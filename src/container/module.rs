use anyhow::Result;
use wasmtime;

pub struct Module {}

impl Module {
    fn new() -> Module {
        Module {}
    }
    fn kernel() -> Result<()> {
        let engine = wasmtime::Engine::default();
        let wat = r#"
        (module
            (import "host" "host_func" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
        let module = wasmtime::Module::new(&engine, wat)?;
        let mut linker = wasmtime::Linker::new(&engine);
        linker.func_wrap("host", "host_func", |caller: Caller<'_, u32>, param: i32| {
            println!("Got {} from WebAssembly", param);
            println!("my host state is: {}", caller.data());
        })?;
        let mut store = wasmtime::Store::new(&engine, 0);
        Ok(())
    }
}
