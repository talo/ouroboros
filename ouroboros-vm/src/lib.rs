use std::{ffi::CString, mem::ManuallyDrop, slice};

use ouroboros::Lambda;
use ouroboros_wasm::ErrorCode;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot::{self, Sender as Responder},
};
use wasmtime::{Caller, Config, Engine, Linker, Module, Val};

use crate::store::Store;

pub mod store;

pub enum Event {
    Call(Call),
    Deploy(Deploy),
    Shutdown,
}

pub struct Call {
    name: String,
    args: serde_json::Value,
    responder: Responder<anyhow::Result<serde_json::Value>>,
}

pub struct Deploy {
    func: Func,
}

#[derive(Clone, Debug)]
pub struct Func {
    name: String,
    entrypoint: String,
    code: Vec<u8>,
}

pub struct VM {
    store: Box<dyn Store + Send>,
    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
}

impl VM {
    pub fn new(store: Box<dyn Store + Send>) -> (Self, Sender<Event>) {
        let (event_sender, event_receiver) = tokio::sync::mpsc::channel(1024);
        (
            Self {
                store,
                event_receiver,
                event_sender: event_sender.clone(),
            },
            event_sender,
        )
    }

    pub async fn run(mut self) {
        while let Some(event) = self.event_receiver.recv().await {
            match event {
                Event::Call(call_fn) => self.call_fn(call_fn).await,
                Event::Deploy(deploy_fn) => self.deploy_fn(deploy_fn).await,
                Event::Shutdown => {
                    break;
                }
            }
        }
    }

    async fn call_fn(&mut self, call_fn: Call) {
        let Call {
            name,
            args,
            responder,
        } = call_fn;

        let func = match self.store.get_func(&name).await {
            Ok(Some(func)) => func.clone(),
            _ => {
                todo!()
            }
        };

        let event_sender = self.event_sender.clone();
        tokio::spawn(async move {
            let ret = Self::call_in_background(func, args, event_sender).await;

            if let Err(_e) = responder.send(ret) {
                tracing::error!("`VM::call_fn` response channel is closed");
            }
        });
    }

    async fn deploy_fn(&mut self, deploy_fn: Deploy) {
        let Deploy { func } = deploy_fn;

        if let Err(_e) = self.store.insert_func(func).await {
            todo!()
        }
    }

    async fn call_in_background(
        func: Func,
        args: serde_json::Value,
        event_sender: Sender<Event>,
    ) -> anyhow::Result<serde_json::Value> {
        use wasmtime::Store;

        // Create a new instance
        let engine = Engine::new(Config::default().async_support(true))?;
        let mut store = Store::new(&engine, event_sender);
        let module = Module::new(&engine, &func.code)?;
        let mut linker = Linker::new(&engine);
        linker.func_wrap7_async(
                "env",
                "__ouroboros__call_fn",
                move |mut caller: Caller<'_, Sender<Event>>,
                      lambda_ptr: i32,
                      lambda_size: i32,
                      args_ptr: i32,
                      args_size: i32,
                      out_ret: i32,
                      out_ret_size: i32,
                      out_err_code: i32| {
                    println!("> __ouroboros__call_fn");

                    // Extract sender
                    let sender = caller.data().clone();

                    // Load memory
                    let memory = caller
                        .get_export("memory")
                        .expect("wasm memory is unavailable")
                        .into_memory()
                        .expect("wasm memory is not memory");

                    Box::new(async move {
                        { // Check the memory bounds of all pointers
                            if !Self::is_ptr_valid(memory.data_mut(&mut caller), lambda_ptr, lambda_size) {
                                tracing::debug!("lambda pointer is out of bounds");
                                Self::write_err_code_to_memory(
                                    memory.data_mut(&mut caller),
                                    out_err_code,
                                    ErrorCode::MemoryOutOfBounds as i32,
                                );
                                return;
                            }
                            if !Self::is_ptr_valid(memory.data_mut(&mut caller), args_ptr, args_size) {
                                tracing::debug!("args pointer is out of bounds");
                                Self::write_err_code_to_memory(
                                    memory.data_mut(&mut caller),
                                    out_err_code,
                                    ErrorCode::MemoryOutOfBounds as i32,
                                );
                                return;
                            }
                            if !Self::is_ptr_valid(memory.data_mut(&mut caller), out_ret, 4) {
                                tracing::debug!("return pointer is out of bounds");
                                Self::write_err_code_to_memory(
                                    memory.data_mut(&mut caller),
                                    out_err_code,
                                    ErrorCode::MemoryOutOfBounds as i32,
                                );
                                return;
                            }
                            if !Self::is_ptr_valid(memory.data_mut(&mut caller), out_ret_size, 4) {
                                tracing::debug!("return pointer is out of bounds");
                                Self::write_err_code_to_memory(
                                    memory.data_mut(&mut caller),
                                    out_err_code,
                                    ErrorCode::MemoryOutOfBounds as i32,
                                );
                                return;
                            }
                            if !Self::is_ptr_valid(memory.data_mut(&mut caller), out_err_code, 4) {
                                // If the err code pointer is out of bounds, then we
                                // cannot report errs so we do the only thing we
                                // can: fail silently
                                tracing::debug!("error code pointer is out of bounds");
                                return;
                            }
                        }

                        // Parse arguments, call the function, and process the response
                        let ret = {
                            let data = memory.data_mut(&mut caller);

                            // Load the function name as a utf8 string
                            let lambda = String::from_utf8(unsafe {
                                Vec::from_raw_parts(
                                    data[lambda_ptr as usize..].as_mut_ptr(),
                                    lambda_size as usize,
                                    lambda_size as usize,
                                )
                            })
                            .map(ManuallyDrop::new);

                            // If the string is invalid, then report the error and
                            // stop
                            let lambda = if let Ok(lambda) = lambda {
                                match serde_json::from_str::<
                                    Lambda<serde_json::Value, serde_json::Value>,
                                >(lambda.as_str())
                                {
                                    Ok(lambda) => lambda,
                                    Err(_) => {
                                        Self::write_err_code_to_memory(
                                            data,
                                            out_err_code,
                                            ErrorCode::InvalidJson as i32,
                                        );
                                        return;
                                    }
                                }
                            } else {
                                Self::write_err_code_to_memory(
                                    data,
                                    out_err_code,
                                    ErrorCode::InvalidUtf8 as i32,
                                );
                                return;
                            };

                            println!("> __ouroboros__call_fn: lambda={:?}", lambda);

                            // Parse the args
                            let args = ManuallyDrop::new(unsafe {
                                slice::from_raw_parts(
                                    data[(args_ptr as usize)..].as_mut_ptr(),
                                    args_size as usize,
                                )
                            });
                            let args = match serde_json::from_slice::<serde_json::Value>(&args) {
                                Ok(args) => args,
                                Err(_) => {
                                    Self::write_err_code_to_memory(
                                        data,
                                        out_err_code,
                                        ErrorCode::InvalidJson as i32,
                                    );
                                    return;
                                }
                            };

                            println!("> __ouroboros__call_fn: args={:?}", args);

                            // FIXME: Need a more intelligent merging of the args
                            // with the captured args. Because the args could
                            // represent multiple elements of a tuple.
                            let mut captured_args = lambda.captured_args;
                            captured_args.push(args);

                            // Call the function
                            let (responder, response) = oneshot::channel();
                            let send_result = sender
                                .send(Event::Call(Call {
                                    name: lambda.n,
                                    args: serde_json::to_value(&captured_args).unwrap(), // FIXME: Bad unwrap
                                    responder,
                                }))
                                .await;
                            if let Err(_e) = send_result {
                                tracing::error!("`VM::call_fn` event channel is closed");
                                Self::write_err_code_to_memory(
                                    data,
                                    out_err_code,
                                    ErrorCode::Internal as i32,
                                );
                                return;
                            }
                            match response.await {
                                Ok(ret) => match ret {
                                    Ok(ret) => ret,
                                    Err(e) => {
                                        tracing::error!("`VM::call_fn` bad response: {}", e);
                                        Self::write_err_code_to_memory(
                                            data,
                                            out_err_code,
                                            ErrorCode::Internal as i32,
                                        );
                                        return;
                                    }
                                },
                                Err(_e) => {
                                    tracing::error!("`VM::call_fn` response channel is closed");
                                    Self::write_err_code_to_memory(
                                        data,
                                        out_err_code,
                                        ErrorCode::Internal as i32,
                                    );
                                    return;
                                }
                            }
                        };

                        let ret_json = match serde_json::to_vec(&ret) {
                            Ok(ret_json) => ret_json,
                            Err(_) => {
                                let data = memory.data_mut(&mut caller);
                                Self::write_err_code_to_memory(
                                    data,
                                    out_err_code,
                                    ErrorCode::Internal as i32,
                                );
                                return;
                            }
                        };
                        let ret_json_size = ret_json.len();

                        let ret_ptr = match Self::alloc_mem(&mut caller, ret_json_size as i32).await {
                            Some(ret_ptr) => ret_ptr,
                            None => {
                                Self::write_err_code_to_memory(
                                    memory.data_mut(&mut caller),
                                    out_err_code,
                                    ErrorCode::MemoryOutOfBounds as i32,
                                );
                                return;
                            }
                        };

                        let data = memory.data_mut(&mut caller);
                        data[ret_ptr as usize..ret_ptr as usize + ret_json_size]
                            .copy_from_slice(ret_json.as_slice());
                        data[out_ret as usize..out_ret as usize + 4]
                            .copy_from_slice(&ret_ptr.to_le_bytes());
                        data[out_ret_size as usize..out_ret_size as usize + 4]
                            .copy_from_slice(&(ret_json_size as i32).to_le_bytes());
                    })
                },
            )?;

        let instance = linker.instantiate_async(&mut store, &module).await?;

        // Load the standard library
        let alloc = instance.get_typed_func::<i32, i32>(&mut store, "__ouroboros__alloc")?;
        let free = instance.get_typed_func::<(i32, i32), ()>(&mut store, "__ouroboros__free")?;

        // Load the entrypoint
        let entrypoint = instance.get_typed_func::<i32, i32>(&mut store, &func.entrypoint)?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("memory must be available");

        // Alloc enough memory for the args and inject them into memory
        let args = CString::new(serde_json::to_string(&args)?)?;
        let args_size = args.as_bytes_with_nul().len();
        let args_ptr = alloc.call_async(&mut store, args_size as i32).await?; // FIXME: We cannot blindly trust the allocated pointer
        let data = &mut memory.data_mut(&mut store);
        data[(args_ptr as usize)..(args_ptr as usize + args_size)]
            .copy_from_slice(args.as_bytes_with_nul());

        // Load the return value
        let ret_ptr = entrypoint.call_async(&mut store, args_ptr).await?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("memory must be available");
        let data = &mut memory.data_mut(&mut store)[(ret_ptr as usize)..]; // FIXME: We cannot blindly trust that the return C string is actually null terminated
        let ret = serde_json::from_slice(
            ManuallyDrop::new(unsafe { CString::from_raw(data.as_mut_ptr() as *mut i8) })
                .as_bytes(),
        )?;

        // Free the allocated memory
        free.call_async(&mut store, (args_ptr, args_size as i32))
            .await?;

        Ok(ret)
    }

    /// Writes an `ErrCode` to a slice of data. This slice of data is assumed to
    /// represent the linear memory of a WASM module.
    fn write_err_code_to_memory(mem: &mut [u8], index: i32, err_code: i32) {
        tracing::debug!("`__ouroboros__call_fn` error code: {}", err_code);

        mem[index as usize..index as usize + 4].copy_from_slice(&err_code.to_le_bytes());
    }

    /// Check the memory bounds of a slice of data. This slice of data is
    /// assumed to represent the linear memory of a WASM module. Returns true if
    /// the memory bounds are valid.
    fn is_ptr_valid(mem: &[u8], ptr: i32, size: i32) -> bool {
        (ptr as usize) < mem.len() && (ptr + size) as usize <= mem.len()
    }

    /// Allocate memory for a WASM module. Returns the pointer to the allocated
    /// memory. Returns `None` if the allocated memory is invalid.
    async fn alloc_mem<T>(mut caller: &mut Caller<'_, T>, size: i32) -> Option<i32>
    where
        T: Send,
    {
        let memory = caller
            .get_export("memory")
            .expect("memory must be unavailable")
            .into_memory()
            .expect("memory must be memory");

        let alloc = caller
            .get_export("__ouroboros__alloc")
            .expect("alloc must be unavailable")
            .into_func()
            .expect("alloc must be a function");

        let mut ptr = [Val::I32(0)];
        alloc
            .call_async(&mut caller, &[Val::I32(size)], &mut ptr)
            .await
            .expect("async call should succeed"); // FIXME: Is it ok to expect this call to succeed?
        let ptr = ptr[0].unwrap_i32();

        Self::is_ptr_valid(memory.data(caller), ptr, size).then_some(ptr)
    }
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
