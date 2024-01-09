#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    use std::{ffi::CString, mem::ManuallyDrop, slice};

    use ouroboros::{Lambda, A, B};
    use ouroboros_wasm::ErrorCode;
    use wasmtime::{Caller, Config, Engine, Linker, Module, Store, Val};

    fn write_err_code_to_memory(data: &mut [u8], index: i32, err_code: i32) {
        data[index as usize..(index + 4) as usize].copy_from_slice(&err_code.to_le_bytes());
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        // Modules can be compiled through either the text or binary format
        let engine = Engine::new(Config::default().async_support(true))?;
        let code =
            include_bytes!("../../target/wasm32-unknown-unknown/debug/ouroboros_vm_prelude.wasm");
        let module = Module::new(&engine, code)?;

        // Create a `Linker` which will be later used to instantiate this module.
        // Host functionality is defined by name within the `Linker`.
        let mut linker = Linker::new(&engine);

        // All wasm objects operate within the context of a "store". Each
        // `Store` has a type parameter to store host-specific data, which in
        // this case we're using `4` for.
        let mut store = Store::new(&engine, ());

        linker.func_wrap9_async(
            "env",
            "__ouroboros__call_fn",
            move |mut caller: Caller<'_, ()>,
                  name_ptr: i32,
                  name_size: i32,
                  args_ptr: i32,
                  args_size: i32,
                  extras_ptr: i32,
                  extras_size: i32,
                  out_result: i32,
                  out_result_size: i32,
                  out_err_code: i32| {
                Box::new(async move {
                    // Load memory
                    let memory = caller
                        .get_export("memory")
                        .expect("wasm memory is unavailable")
                        .into_memory()
                        .expect("wasm memory is not memory");

                    {
                        // Memory checks
                        let data = memory.data_mut(&mut caller);

                        if out_err_code as usize + 4 > data.len() {
                            // If the err code pointer is out of bounds, then we
                            // cannot report errs so we do the only thing we
                            // can: fail silently
                            return;
                        }

                        if name_ptr as usize + 4 > data.len() {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }
                        if (name_ptr + name_size) as usize + 4 > data.len() {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryRangeOutOfBounds as i32,
                            );
                            return;
                        }

                        if args_ptr as usize + 4 > data.len() {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }
                        if (args_ptr + args_size) as usize + 4 > data.len() {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }

                        if extras_ptr as usize + 4 > data.len() {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }
                        if (extras_ptr + extras_size) as usize + 4 > data.len() {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }

                        // This is where we will store the result pointer
                        if out_result as usize + 4 > data.len() {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }
                        // This is where we will store the result size
                        if out_result_size as usize + 4 > data.len() {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::MemoryIndexOutOfBounds as i32,
                            );
                            return;
                        }
                    }

                    let result = {
                        let data = memory.data_mut(&mut caller);

                        // Load the function name as a utf8 string
                        let name = String::from_utf8(unsafe {
                            Vec::from_raw_parts(
                                data[(name_ptr as usize)..].as_mut_ptr(),
                                (name_size) as usize,
                                (name_size) as usize,
                            )
                        })
                        .map(ManuallyDrop::new);

                        // If the string is invalid, then report the error and
                        // stop
                        let name = if let Ok(name) = name {
                            name
                        } else {
                            write_err_code_to_memory(
                                data,
                                out_err_code,
                                ErrorCode::InvalidUtf8 as i32,
                            );
                            return;
                        };

                        let args_json = ManuallyDrop::new(unsafe {
                            slice::from_raw_parts(
                                data[(args_ptr as usize)..].as_mut_ptr(),
                                args_size as usize,
                            )
                        });
                        let extras_json = ManuallyDrop::new(unsafe {
                            slice::from_raw_parts(
                                data[(extras_ptr as usize)..].as_mut_ptr(),
                                extras_size as usize,
                            )
                        });

                        let args = match serde_json::from_slice::<u32>(&args_json) {
                            Ok(args) => args,
                            Err(_) => {
                                write_err_code_to_memory(
                                    data,
                                    out_err_code,
                                    ErrorCode::InvalidUtf8 as i32,
                                );
                                return;
                            }
                        };
                        println!("calling function `{}({})`", name.as_str(), args);

                        2 * args
                    };

                    let result_json = serde_json::to_vec(&result).expect("invalid fn result");
                    let result_json_size = result_json.len();

                    let alloc = caller
                        .get_export("__ouroboros__alloc")
                        .expect("alloc is unavailable")
                        .into_func()
                        .expect("alloc is not a func");

                    let mut result_data_ptr = [Val::I32(0)];
                    alloc
                        .call_async(
                            &mut caller,
                            &[Val::I32(result_json_size as i32)],
                            &mut result_data_ptr,
                        )
                        .await
                        .expect("async call failed");
                    let result_data_ptr = result_data_ptr[0].unwrap_i32();

                    let data = &mut memory.data_mut(&mut caller);

                    data[result_data_ptr as usize..result_data_ptr as usize + result_json_size]
                        .copy_from_slice(result_json.as_slice());
                    data[out_result as usize..out_result as usize + 4]
                        .copy_from_slice(&result_data_ptr.to_le_bytes());
                    data[out_result_size as usize..out_result_size as usize + 4]
                        .copy_from_slice(&(result_json_size as i32).to_le_bytes());
                })
            },
        )?;

        let instance = linker.instantiate_async(&mut store, &module).await?;

        // Get the standard library
        let alloc = instance.get_typed_func::<i32, i32>(&mut store, "__ouroboros__alloc")?;
        let free = instance.get_typed_func::<(i32, i32), ()>(&mut store, "__ouroboros__free")?;

        // Get the `map` function
        let map = instance.get_typed_func::<i32, i32>(&mut store, "__entrypoint__map")?;
        let Some(memory) = instance.get_memory(&mut store, "memory") else {
            anyhow::bail!("wasm memory is unavailable");
        };

        // // Setup arguments for the `map` function
        let map_args = (
            Lambda::<A, B>::new("f"),
            vec![A::new(&1), A::new(&2), A::new(&3)],
        );
        let map_args_json = serde_json::to_string(&map_args).expect("invalid json");
        let map_args_json_str = CString::new(map_args_json).expect("must not contain nul bytes");
        println!("{}", map_args_json_str.to_str().expect("invalid utf8"));

        // Alloc enough memory for the JSON string and inject it into memory
        let map_args_json_str_size = map_args_json_str.as_bytes_with_nul().len();
        let map_args_json_str_ptr = alloc
            .call_async(&mut store, map_args_json_str_size as i32)
            .await?;
        let data = &mut memory.data_mut(&mut store)[(map_args_json_str_ptr as usize)..];
        data[..map_args_json_str_size].copy_from_slice(map_args_json_str.as_bytes_with_nul());

        println!("introspecting `map`...");
        let result_ptr = map.call_async(&mut store, map_args_json_str_ptr).await?;

        let Some(memory) = instance.get_memory(&mut store, "memory") else {
            anyhow::bail!("wasm memory is unavailable");
        };

        let data = &mut memory.data_mut(&mut store)[(result_ptr as usize)..];
        let result = ManuallyDrop::new(unsafe { CString::from_raw(data.as_mut_ptr() as *mut i8) });
        println!("> {}", result.to_str()?);

        free.call_async(
            &mut store,
            (map_args_json_str_ptr, map_args_json_str_size as i32),
        )
        .await?;

        Ok(())
    }
}
