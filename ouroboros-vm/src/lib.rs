use std::{ffi::CString, future::Future, mem::ManuallyDrop, slice};

use ouroboros::Lambda;
use ouroboros_wasm::ErrorCode;
use serde::{Deserialize, Serialize};
use wasmtime::{Caller, Config, Engine, Linker, Module, Store, Val};

pub async fn thunk_in_background<S, A, R, Fut>(
    bytecode: impl AsRef<[u8]>,
    entrypoint: impl AsRef<str>,
    initial_state: S,
    args: A,
    cb: impl 'static + Clone + Fn(&mut Caller<'_, S>, Lambda<A, R>, A) -> Fut + Send + Sync,
) -> anyhow::Result<R>
where
    S: Send,
    for<'de> A: Deserialize<'de> + Serialize,
    for<'de> R: Deserialize<'de>,
    Fut: Future<Output = ()> + Send + Unpin,
{
    let engine = Engine::new(Config::default().async_support(true))?;
    let module = Module::new(&engine, bytecode)?;

    let mut store = Store::new(&engine, initial_state);
    let mut linker = Linker::new(&engine);

    linker.func_wrap8_async(
        "env",
        "__ouroboros__call",
        move |mut caller: Caller<'_, S>,
              version: i32,
              lambda_ptr: i32,
              lambda_size: i32,
              args_ptr: i32,
              args_size: i32,
              out_ret: i32,
              out_ret_size: i32,
              out_err_code: i32| {
            tracing::debug!("> __ouroboros__call");

            let cb = cb.clone();

            Box::new(async move {
                // Parse arguments, call the function, and process the response
                let (lambda, args) = {
                    let mem_data = mem_data(&mut caller);

                    check_memory_bounds_and_report_errs(
                        mem_data,
                        lambda_ptr,
                        lambda_size,
                        args_ptr,
                        args_size,
                        out_ret,
                        out_ret_size,
                        out_err_code,
                    );

                    if version != 1 {
                        write_err_code(mem_data, out_err_code, ErrorCode::InvalidVersion as i32);
                    }

                    // Load the function name as a utf8 string
                    let lambda = ManuallyDrop::new(unsafe {
                        slice::from_raw_parts(
                            mem_data[(lambda_ptr as usize)..].as_mut_ptr(),
                            lambda_size as usize,
                        )
                    });
                    let lambda = match serde_json::from_slice::<Lambda<A, R>>(&lambda) {
                        Ok(lambda) => lambda,
                        Err(_) => {
                            write_err_code(mem_data, out_err_code, ErrorCode::InvalidJson as i32);
                            return;
                        }
                    };

                    // Parse the args
                    let args = ManuallyDrop::new(unsafe {
                        slice::from_raw_parts(
                            mem_data[(args_ptr as usize)..].as_mut_ptr(),
                            args_size as usize,
                        )
                    });
                    let args = match serde_json::from_slice::<A>(&args) {
                        Ok(args) => args,
                        Err(_) => {
                            write_err_code(mem_data, out_err_code, ErrorCode::InvalidJson as i32);
                            return;
                        }
                    };

                    (lambda, args)
                };

                let ret = cb(&mut caller, lambda, args).await;

                let ret_json = match serde_json::to_vec(&ret) {
                    Ok(ret_json) => ret_json,
                    Err(_) => {
                        write_err_code(
                            mem_data(&mut caller),
                            out_err_code,
                            ErrorCode::Internal as i32,
                        );
                        return;
                    }
                };
                let ret_json_size = ret_json.len();

                let ret_ptr = match alloc_mem(&mut caller, ret_json_size as i32).await {
                    Some(ret_ptr) => ret_ptr,
                    None => {
                        write_err_code(
                            mem_data(&mut caller),
                            out_err_code,
                            ErrorCode::MemoryOutOfBounds as i32,
                        );
                        return;
                    }
                };

                let mem_data = mem_data(&mut caller);

                mem_data[ret_ptr as usize..ret_ptr as usize + ret_json_size]
                    .copy_from_slice(ret_json.as_slice());
                mem_data[out_ret as usize..out_ret as usize + 4]
                    .copy_from_slice(&ret_ptr.to_le_bytes());
                mem_data[out_ret_size as usize..out_ret_size as usize + 4]
                    .copy_from_slice(&(ret_json_size as i32).to_le_bytes());
            })
        },
    )?;

    // Instantiate the module instance (this must be done after linking all host
    // functions)
    let instance = linker.instantiate_async(&mut store, &module).await?;

    // Load the __ouroboros__ interface
    let entrypoint = instance.get_typed_func::<i32, i32>(&mut store, entrypoint.as_ref())?;
    let alloc = instance.get_typed_func::<i32, i32>(&mut store, "__ouroboros__alloc")?;
    let free = instance.get_typed_func::<(i32, i32), ()>(&mut store, "__ouroboros__free")?;

    // Alloc enough memory for the args
    let args = CString::new(serde_json::to_string(&args)?)?;
    let args_size = args.as_bytes_with_nul().len();
    let args_ptr = alloc.call_async(&mut store, args_size as i32).await?;

    // Check that the allocated args pointer is actuall valid
    if !is_ptr_valid(
        instance
            .get_memory(&mut store, "memory")
            .expect("memory must be available")
            .data_mut(&mut store),
        args_ptr,
        args_size as i32,
    ) {
        return Err(anyhow::anyhow!("invalid memory pointer"));
    }

    // Copy the args into memory
    instance
        .get_memory(&mut store, "memory")
        .expect("memory must be available")
        .data_mut(&mut store)[(args_ptr as usize)..(args_ptr as usize + args_size)]
        .copy_from_slice(args.as_bytes_with_nul());

    // Load the return value
    let ret_ptr = entrypoint.call_async(&mut store, args_ptr).await?;
    let ret_data = &mut instance
        .get_memory(&mut store, "memory")
        .expect("memory must be available")
        .data_mut(&mut store)[(ret_ptr as usize)..]; // FIXME: We cannot blindly trust that the return C string is actually null terminated
    let ret = serde_json::from_slice(
        ManuallyDrop::new(unsafe { CString::from_raw(ret_data.as_mut_ptr() as *mut i8) })
            .as_bytes(),
    )?;

    // Free the allocated memory
    free.call_async(&mut store, (args_ptr, args_size as i32))
        .await?;

    Ok(ret)
}

/// Helper function for getting the underlying data of the WASM memory.
pub fn mem_data<'a, T>(caller: &'a mut Caller<'_, T>) -> &'a mut [u8] {
    caller
        .get_export("memory")
        .expect("wasm memory is unavailable")
        .into_memory()
        .expect("wasm memory is not memory")
        .data_mut(caller)
}

/// Allocate memory for a WASM module. Returns the pointer to the allocated
/// memory. Returns `None` if the allocated memory is invalid.
pub async fn alloc_mem<T>(mut caller: &mut Caller<'_, T>, size: i32) -> Option<i32>
where
    T: Send,
{
    let memory = caller
        .get_export("memory")
        .expect("memory must be unavailable")
        .into_memory()
        .expect("memory must be a memory block");

    let alloc = caller
        .get_export("__ouroboros__alloc")
        .expect("alloc must be unavailable")
        .into_func()
        .expect("alloc must be a function");

    let mut ptr = [Val::I32(0)];
    alloc
        .call_async(&mut caller, &[Val::I32(size)], &mut ptr)
        .await
        .expect("async must be ok"); // FIXME: Is it ok to expect this call to succeed?
    let ptr = ptr[0].unwrap_i32();

    is_ptr_valid(memory.data(caller), ptr, size).then_some(ptr)
}

/// Check all of the input and output pointers. They must point to somewhere in
/// the WASM memory and their size must not cause them to overflow.
#[allow(clippy::too_many_arguments)]
pub fn check_memory_bounds_and_report_errs(
    mem_data: &mut [u8],
    lambda_ptr: i32,
    lambda_size: i32,
    args_ptr: i32,
    args_size: i32,
    out_ret: i32,
    out_ret_size: i32,
    out_err_code: i32,
) -> bool {
    // Check the memory pointed to by the err code pointer.
    if !is_ptr_valid(mem_data, out_err_code, 4) {
        // If the err code pointer is out of bounds, then we
        // cannot report errs so we do the only thing we
        // can: fail silently.
        tracing::debug!("error code pointer is out of bounds");
        return false;
    }

    // Check the memory bounds of all pointers
    if !is_ptr_valid(mem_data, lambda_ptr, lambda_size) {
        tracing::debug!("lambda pointer is out of bounds");
        write_err_code(mem_data, out_err_code, ErrorCode::MemoryOutOfBounds as i32);
        return false;
    }
    if !is_ptr_valid(mem_data, args_ptr, args_size) {
        tracing::debug!("args pointer is out of bounds");
        write_err_code(mem_data, out_err_code, ErrorCode::MemoryOutOfBounds as i32);
        return false;
    }
    if !is_ptr_valid(mem_data, out_ret, 4) {
        tracing::debug!("return pointer is out of bounds");
        write_err_code(mem_data, out_err_code, ErrorCode::MemoryOutOfBounds as i32);
        return false;
    }
    if !is_ptr_valid(mem_data, out_ret_size, 4) {
        tracing::debug!("return pointer is out of bounds");
        write_err_code(mem_data, out_err_code, ErrorCode::MemoryOutOfBounds as i32);
        return false;
    }

    true
}

/// Writes an `ErrCode` to a slice of data. This slice of data is assumed to
/// represent the linear memory of a WASM module.
pub fn write_err_code(mem_data: &mut [u8], out_err_code: i32, err_code: i32) {
    tracing::debug!("`__ouroboros__call` error code: {}", err_code);

    mem_data[out_err_code as usize..out_err_code as usize + 4]
        .copy_from_slice(&err_code.to_le_bytes());
}

/// Check the memory bounds of a slice of data. This slice of data is assumed to
/// represent the linear memory of a WASM module. Returns true if the memory
/// bounds are valid.
pub fn is_ptr_valid(mem_data: &[u8], ptr: i32, size: i32) -> bool {
    if ptr < 0 {
        return false;
    }
    if size < 1 {
        return false;
    }
    (ptr as usize) < mem_data.len() && (ptr + size) as usize <= mem_data.len()
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod test {

    use ouroboros::{Lambda, A, B};

    use crate::{store::InMemoryStore, Call, Deploy, Event, Func, VM};

    #[tokio::test]
    async fn test_map() -> anyhow::Result<()> {
        let (vm, vm_sender) = VM::new(Box::new(InMemoryStore::new()));

        let vm_run_handle = tokio::spawn(async { vm.run().await });

        vm_sender
            .send(Event::Deploy(Deploy {
                func: Func {
                    name: "mul_u32".to_string(),
                    entrypoint: "__entrypoint__mul_u32".to_string(),
                    code: include_bytes!(
                        "../../target/wasm32-unknown-unknown/debug/ouroboros_vm_prelude.wasm"
                    )
                    .to_vec(),
                },
            }))
            .await?;

        vm_sender
            .send(Event::Deploy(Deploy {
                func: Func {
                    name: "map".to_string(),
                    entrypoint: "__entrypoint__map".to_string(),
                    code: include_bytes!(
                        "../../target/wasm32-unknown-unknown/debug/ouroboros_vm_prelude.wasm"
                    )
                    .to_vec(),
                },
            }))
            .await?;

        let map_args = (
            Lambda::<A, B>::with_captured_args("mul_u32", vec![serde_json::json!(2u32)]),
            vec![A::new(&1u32), A::new(&2u32), A::new(&3u32)],
        );
        let map_args_json = serde_json::to_value(&map_args).expect("invalid json");

        println!("$ external_call: map");
        let (responder, response) = tokio::sync::oneshot::channel();
        vm_sender
            .send(Event::Call(Call {
                name: "map".to_string(),
                args: map_args_json,
                responder,
            }))
            .await?;

        let resp = response.await?;
        println!("$ external_call: ret={:?}", resp);

        vm_sender.send(Event::Shutdown).await?;
        vm_run_handle.await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_compose() -> anyhow::Result<()> {
        let (vm, vm_sender) = VM::new(Box::new(InMemoryStore::new()));

        let vm_run_handle = tokio::spawn(async { vm.run().await });

        vm_sender
            .send(Event::Deploy(Deploy {
                func: Func {
                    name: "compose".to_string(),
                    entrypoint: "__entrypoint__compose".to_string(),
                    code: include_bytes!(
                        "../../target/wasm32-unknown-unknown/debug/ouroboros_vm_prelude.wasm"
                    )
                    .to_vec(),
                },
            }))
            .await?;

        vm_sender
            .send(Event::Deploy(Deploy {
                func: Func {
                    name: "mul_u32".to_string(),
                    entrypoint: "__entrypoint__mul_u32".to_string(),
                    code: include_bytes!(
                        "../../target/wasm32-unknown-unknown/debug/ouroboros_vm_prelude.wasm"
                    )
                    .to_vec(),
                },
            }))
            .await?;

        vm_sender
            .send(Event::Deploy(Deploy {
                func: Func {
                    name: "map".to_string(),
                    entrypoint: "__entrypoint__map".to_string(),
                    code: include_bytes!(
                        "../../target/wasm32-unknown-unknown/debug/ouroboros_vm_prelude.wasm"
                    )
                    .to_vec(),
                },
            }))
            .await?;

        vm_sender
            .send(Event::Deploy(Deploy {
                func: Func {
                    name: "take".to_string(),
                    entrypoint: "__entrypoint__take".to_string(),
                    code: include_bytes!(
                        "../../target/wasm32-unknown-unknown/debug/ouroboros_vm_prelude.wasm"
                    )
                    .to_vec(),
                },
            }))
            .await?;

        let compose_args = (
            Lambda::<B, B>::with_captured_args("take", vec![serde_json::json!(3u32)]),
            Lambda::<A, B>::with_captured_args(
                "map",
                vec![serde_json::json!(Lambda::<u32, u32>::with_captured_args(
                    "mul_u32",
                    vec![serde_json::json!(2u32)],
                ))],
            ),
            vec![A::new(&1u32), A::new(&2u32), A::new(&3u32), A::new(&4u32)],
        );
        let compose_args_json = serde_json::to_value(&compose_args).expect("invalid json");

        println!("$ external_call: compose");
        let (responder, response) = tokio::sync::oneshot::channel();
        vm_sender
            .send(Event::Call(Call {
                name: "compose".to_string(),
                args: compose_args_json,
                responder,
            }))
            .await?;

        let resp = response.await?;
        println!("$ external_call: ret={:?}", resp);

        vm_sender.send(Event::Shutdown).await?;
        vm_run_handle.await?;

        Ok(())
    }
}
