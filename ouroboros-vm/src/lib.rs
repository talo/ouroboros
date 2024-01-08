#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, ffi::CString, mem::ManuallyDrop};

    use wasmtime::{Caller, Config, Engine, Linker, Module, Store};

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        // Modules can be compiled through either the text or binary format
        let engine = Engine::new(Config::default().async_support(true))?;
        let wasm =
            include_bytes!("../../target/wasm32-unknown-unknown/release/ouroboros_vm_stdlib.wasm");
        let module = Module::new(&engine, wasm)?;

        // Create a `Linker` which will be later used to instantiate this module.
        // Host functionality is defined by name within the `Linker`.
        let mut linker = Linker::new(&engine);
        linker.func_wrap1_async(
            "ouroboros",
            "call_mod",
            |caller: Caller<'_, u32>, _param: i32| {
                Box::new(async move {
                    println!("SPAWN from WebAssembly");
                    println!("my host state is: {}", caller.data());
                    ()
                })
            },
        )?;
        linker.func_wrap1_async(
            "ouroboros",
            "call_fn",
            |caller: Caller<'_, u32>, _param: i32| {
                Box::new(async move {
                    println!("SPAWN from WebAssembly");
                    println!("my host state is: {}", caller.data());
                    ()
                })
            },
        )?;

        // All wasm objects operate within the context of a "store". Each
        // `Store` has a type parameter to store host-specific data, which in
        // this case we're using `4` for.
        let mut store = Store::new(&engine, 4);
        let instance = linker.instantiate_async(&mut store, &module).await?;
        let alloc = instance.get_typed_func::<i32, i32>(&mut store, "alloc")?;
        let free = instance.get_typed_func::<(i32, i32), ()>(&mut store, "free")?;
        let echo = instance.get_typed_func::<i32, i32>(&mut store, "echo")?;
        let Some(memory) = instance.get_memory(&mut store, "memory") else {
            anyhow::bail!("Failed to get WASM memory");
        };

        // Build a JSON serialized map of "foo" -> "bar!"
        let mut hello_world = BTreeMap::new();
        hello_world.insert("foo".to_string(), "bar!".to_string());
        let hello_world = serde_json::to_string(&hello_world).expect("invalid json");
        let hello_world = CString::new(hello_world).unwrap();

        // Alloc enough memory for the JSON string and inject it into memory
        let size = hello_world.as_bytes_with_nul().len();
        let hello_world_ptr = alloc.call_async(&mut store, size as i32).await?;
        let data = &mut memory.data_mut(&mut store)[(hello_world_ptr as usize)..];
        data[..size].copy_from_slice(hello_world.as_bytes_with_nul());

        println!("hello_world = {}", hello_world_ptr);
        println!("calling function...");
        let result_ptr = echo.call_async(&mut store, hello_world_ptr).await?;
        println!("result_ptr = {}", result_ptr);

        let Some(memory) = instance.get_memory(&mut store, "memory") else {
            anyhow::bail!("Failed to get WASM memory");
        };

        let data = &mut memory.data_mut(&mut store)[(result_ptr as usize)..];
        let result = ManuallyDrop::new(unsafe { CString::from_raw(data.as_mut_ptr() as *mut i8) });
        println!("result = {}", result.to_str()?);

        free.call_async(&mut store, (hello_world_ptr, size as i32))
            .await?;

        Ok(())
    }
}
