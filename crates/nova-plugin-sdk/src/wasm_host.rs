use crate::{PluginError, PluginRequest, PluginResponse, Result};
use wasmtime::{Engine, Instance, Module, Store};
use wasmtime_wasi::{preview1, WasiCtxBuilder};

/// WASM plugin host environment
pub struct WasmHost {
    engine: Engine,
}

impl WasmHost {
    pub fn new() -> Result<Self> {
        let engine = Engine::default();
        Ok(Self { engine })
    }

    /// Load a WASM module from bytes
    pub fn load_module(&self, wasm_bytes: &[u8]) -> Result<Module> {
        Module::new(&self.engine, wasm_bytes).map_err(PluginError::WasmError)
    }

    /// Create a new WASM instance with WASI support
    pub fn create_instance(
        &self,
        module: &Module,
    ) -> Result<(Store<preview1::WasiP1Ctx>, Instance)> {
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_env()
            .build_p1();

        let mut store = Store::new(&self.engine, wasi);

        let mut linker = wasmtime::Linker::new(&self.engine);
        preview1::add_to_linker_sync(&mut linker, |s| s)?;

        // Add Nova Design specific host functions
        // self.add_host_functions(&mut linker)?;

        let instance = linker.instantiate(&mut store, module)?;

        Ok((store, instance))
    }

    // /// Add Nova Design specific host functions
    // fn add_host_functions(&self, linker: &mut wasmtime::Linker<preview1::WasiP1Ctx>) -> Result<()> {
    //     // Add host function for logging
    //     linker.func_wrap("nova", "log", |_caller: Caller<'_, preview1::WasiP1Ctx>, level: i32, ptr: i32, len: i32| {
    //         // In a real implementation, this would read from WASM memory and log
    //         println!("Plugin log [{}]: message at ptr={}, len={}", level, ptr, len);
    //     })?;

    //     // Add host function for getting project data
    //     linker.func_wrap("nova", "get_project_data", |_caller: Caller<'_, preview1::WasiP1Ctx>| -> i32 {
    //         // In a real implementation, this would serialize and return project data
    //         0 // Success
    //     })?;

    //     // Add host function for setting result data
    //     linker.func_wrap("nova", "set_result", |_caller: Caller<'_, preview1::WasiP1Ctx>, ptr: i32, len: i32| {
    //         // In a real implementation, this would read result data from WASM memory
    //         println!("Plugin result set at ptr={}, len={}", ptr, len);
    //     })?;

    //     Ok(())
    // }

    /// Execute a plugin function
    pub fn execute_plugin(
        &self,
        module: &Module,
        function_name: &str,
        _request: &PluginRequest,
    ) -> Result<PluginResponse> {
        let (mut store, instance) = self.create_instance(module)?;

        // Get the plugin's main function
        let func = instance
            .get_typed_func::<(), i32>(&mut store, function_name)
            .map_err(|e| PluginError::ExecutionError {
                message: format!("Function '{}' not found: {}", function_name, e),
            })?;

        // In a real implementation, we would:
        // 1. Serialize the request to WASM memory
        // 2. Call the function
        // 3. Read the result from WASM memory
        // 4. Deserialize the response

        let result = func
            .call(&mut store, ())
            .map_err(|e| PluginError::ExecutionError {
                message: format!("Plugin execution failed: {}", e),
            })?;

        // For now, return a stub response
        Ok(PluginResponse {
            result: crate::PluginResult {
                success: result == 0,
                message: if result == 0 {
                    "Success".to_string()
                } else {
                    "Error".to_string()
                },
                data: None,
                modified_entities: Vec::new(),
            },
            logs: vec!["WASM plugin executed".to_string()],
        })
    }
}

impl Default for WasmHost {
    fn default() -> Self {
        Self::new().expect("Failed to create WASM host")
    }
}

/// WASM plugin wrapper
pub struct WasmPlugin {
    module: Module,
    metadata: crate::PluginMetadata,
}

impl WasmPlugin {
    pub fn new(wasm_bytes: &[u8], metadata: crate::PluginMetadata) -> Result<Self> {
        let host = WasmHost::new()?;
        let module = host.load_module(wasm_bytes)?;

        Ok(Self { module, metadata })
    }

    pub fn execute(&self, request: &PluginRequest) -> Result<PluginResponse> {
        let host = WasmHost::new()?;
        host.execute_plugin(&self.module, "execute", request)
    }

    pub fn metadata(&self) -> &crate::PluginMetadata {
        &self.metadata
    }
}
