use std::{collections::HashMap, ffi::CString, mem::ManuallyDrop, slice};

use ouroboros::Lambda;
use ouroboros_wasm::ErrorCode;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot::{self, Sender as Responder},
};
use wasmtime::{Caller, Config, Engine, Linker, Module, Store, Val};

pub enum VMEvent {
    Empty,
    Call(Call),
    Deploy(Deploy),
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
    event_receiver: Receiver<VMEvent>,
    event_sender: Sender<VMEvent>,

    fns: HashMap<String, Func>,
}

impl VM {
    pub fn new() -> (Self, Sender<VMEvent>) {
        let (event_sender, event_receiver) = tokio::sync::mpsc::channel(1024);

        (
            Self {
                event_receiver,
                event_sender: event_sender.clone(),
                fns: HashMap::new(),
            },
            event_sender,
        )
    }

    pub async fn run(mut self) {
        while let Some(event) = self.event_receiver.recv().await {
            match event {
                VMEvent::Empty => {}
                VMEvent::Call(call_fn) => self.call_fn(call_fn).await,
                VMEvent::Deploy(deploy_fn) => self.deploy_fn(deploy_fn).await,
            }
        }
    }

    async fn call_fn(&mut self, call_fn: Call) {
        let Call {
            name,
            args,
            responder,
        } = call_fn;

        // Find func
        println!("> {}", name);
        let func = match self.fns.get(&name) {
            Some(func) => func.clone(),
            None => {
                todo!()
            }
        };
        println!("> {}: found", name);

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

        self.fns.insert(func.name.clone(), func); // FIXME: Should we allow overriding previously deployed functions?
    }

    async fn call_in_background(
        func: Func,
        args: serde_json::Value,
        event_sender: Sender<VMEvent>,
    ) -> anyhow::Result<serde_json::Value> {
        // Create a new instance
        let engine = Engine::new(Config::default().async_support(true))?;
        let mut store = Store::new(&engine, event_sender);
        let module = Module::new(&engine, &func.code)?;
        let mut linker = Linker::new(&engine);
        linker.func_wrap7_async(
                "env",
                "__ouroboros__call_fn",
                move |mut caller: Caller<'_, Sender<VMEvent>>,
                      lambda_ptr: i32,
                      lambda_size: i32,
                      args_ptr: i32,
                      args_size: i32,
                      out_result: i32,
                      out_result_size: i32,
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
                        // Memory checks
                        let data = memory.data_mut(&mut caller);

                        if out_err_code as usize + 4 > data.len() {
                            // If the err code pointer is out of bounds, then we
                            // cannot report errs so we do the only thing we
                            // can: fail silently
                            return;
                        }

                        if lambda_ptr as usize + 4 > data.len() {
                            Self::write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }
                        if (lambda_ptr + lambda_size) as usize + 4 > data.len() {
                            Self::write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryRangeOutOfBounds as i32,
                            );
                            return;
                        }

                        if args_ptr as usize + 4 > data.len() {
                            Self::write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }
                        if (args_ptr + args_size) as usize + 4 > data.len() {
                            Self::write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }

                        // This is where we will store the result pointer
                        if out_result as usize + 4 > data.len() {
                            Self::write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }
                        // This is where we will store the result size
                        if out_result_size as usize + 4 > data.len() {
                            Self::write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }

                        // Parse arguments, call the function, and process the response
                        let ret = {
                            let data = memory.data_mut(&mut caller);

                            // Load the function name as a utf8 string
                            let lambda = String::from_utf8(unsafe {
                                Vec::from_raw_parts(
                                    data[(lambda_ptr as usize)..].as_mut_ptr(),
                                    (lambda_size) as usize,
                                    (lambda_size) as usize,
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
                                .send(VMEvent::Call(Call {
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

                        let alloc = caller
                            .get_export("__ouroboros__alloc")
                            .expect("alloc is unavailable")
                            .into_func()
                            .expect("alloc is not a func");

                        let mut ret_ptr = [Val::I32(0)];
                        alloc
                            .call_async(
                                &mut caller,
                                &[Val::I32(ret_json_size as i32)],
                                &mut ret_ptr,
                            )
                            .await
                            .expect("async call failed");
                        let ret_ptr = ret_ptr[0].unwrap_i32();

                        let data = &mut memory.data_mut(&mut caller);

                        // Check that the allocated memory is sufficiently sized to
                        // hold the result
                        if ret_ptr as usize + ret_json_size + 4 > data.len() {
                            Self::write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }

                        data[ret_ptr as usize..ret_ptr as usize + ret_json_size]
                            .copy_from_slice(ret_json.as_slice());
                        data[out_result as usize..out_result as usize + 4]
                            .copy_from_slice(&ret_ptr.to_le_bytes());
                        data[out_result_size as usize..out_result_size as usize + 4]
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
        let Some(memory) = instance.get_memory(&mut store, "memory") else {
            anyhow::bail!("wasm memory is unavailable");
        };

        // Alloc enough memory for the args and inject them into memory
        let args = CString::new(serde_json::to_string(&args)?)?;
        let args_size = args.as_bytes_with_nul().len();
        let args_ptr = alloc.call_async(&mut store, args_size as i32).await?;
        let data = &mut memory.data_mut(&mut store);
        data[(args_ptr as usize)..(args_ptr as usize + args_size)]
            .copy_from_slice(args.as_bytes_with_nul());

        // Load the return value
        let ret_ptr = entrypoint.call_async(&mut store, args_ptr).await?;
        let Some(memory) = instance.get_memory(&mut store, "memory") else {
            anyhow::bail!("wasm memory is unavailable");
        };
        let data = &mut memory.data_mut(&mut store)[(ret_ptr as usize)..]; // FIXME: We cannot blindly trust that the return C string is actually null terminated
        let ret = serde_json::from_slice(
            &ManuallyDrop::new(unsafe { CString::from_raw(data.as_mut_ptr() as *mut i8) })
                .as_bytes(),
        )?;

        // Free the allocated memory
        free.call_async(&mut store, (args_ptr, args_size as i32))
            .await?;

        Ok(ret)
    }

    fn write_err_code_to_memory(data: &mut [u8], index: i32, err_code: i32) {
        tracing::debug!("`__ouroboros__call_fn` error code: {}", err_code);

        data[index as usize..(index + 4) as usize].copy_from_slice(&err_code.to_le_bytes());
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    use std::env;

    use ouroboros::{Lambda, A, B};
    use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

    use crate::{Call, Deploy, Func, VMEvent, VM};

    #[tokio::test]
    async fn test_map() -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(EnvFilter::new(env::var("TRACE").unwrap_or_default()))
            .with(tracing_subscriber::fmt::layer())
            .init();

        let (vm, vm_sender) = VM::new();

        let vm_run_handle = tokio::spawn(async { vm.run().await });

        vm_sender
            .send(VMEvent::Deploy(Deploy {
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
            .send(VMEvent::Deploy(Deploy {
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
            .send(VMEvent::Call(Call {
                name: "map".to_string(),
                args: map_args_json,
                responder,
            }))
            .await?;

        let resp = response.await?;
        println!("$ external_call: ret={:?}", resp);

        vm_run_handle.await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_compose() -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(EnvFilter::new(env::var("TRACE").unwrap_or_default()))
            .with(tracing_subscriber::fmt::layer())
            .init();

        let (vm, vm_sender) = VM::new();

        let vm_run_handle = tokio::spawn(async { vm.run().await });

        vm_sender
            .send(VMEvent::Deploy(Deploy {
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
            .send(VMEvent::Deploy(Deploy {
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
            .send(VMEvent::Deploy(Deploy {
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
            .send(VMEvent::Deploy(Deploy {
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
            .send(VMEvent::Call(Call {
                name: "compose".to_string(),
                args: compose_args_json,
                responder,
            }))
            .await?;

        let resp = response.await?;
        println!("$ external_call: ret={:?}", resp);

        vm_run_handle.await?;

        Ok(())
    }
}
