//! Tree-sitter WASM runtime helpers.

use coco_protocol::{CocoError, CocoResult};
use wasmtime::{Config, Engine, ExternType, Instance, Linker, Module, Store, StoreLimitsBuilder};

/// Runtime limits for WASM grammar execution.
#[derive(Debug, Clone)]
pub struct WasmRuntimeConfig {
    /// Maximum linear memory size in bytes.
    pub max_memory_bytes: u64,
    /// Maximum fuel available to the instance. Zero disables fuel limiting.
    pub max_fuel: u64,
    /// Optional expected language export name.
    pub language_export: Option<String>,
}

impl Default for WasmRuntimeConfig {
    fn default() -> Self {
        Self {
            max_memory_bytes: 64 * 1024 * 1024,
            max_fuel: 5_000_000,
            language_export: None,
        }
    }
}

/// Loaded WASM grammar module and instance metadata.
pub struct WasmGrammar {
    engine: Engine,
    module: Module,
    store: Store<wasmtime::StoreLimits>,
    instance: Instance,
    export_names: Vec<String>,
    language_export: String,
}

impl WasmGrammar {
    /// Loads a grammar module from raw WASM bytes.
    pub fn from_bytes(bytes: &[u8], config: WasmRuntimeConfig) -> CocoResult<Self> {
        if config.max_memory_bytes == 0 {
            return Err(CocoError::user("wasm memory limit must be non-zero"));
        }

        let mut engine_config = Config::new();
        if config.max_fuel > 0 {
            engine_config.consume_fuel(true);
        }
        let engine = Engine::new(&engine_config).map_err(CocoError::compute)?;
        let module = Module::from_binary(&engine, bytes).map_err(CocoError::compute)?;

        let max_memory_bytes = usize::try_from(config.max_memory_bytes)
            .map_err(|_| CocoError::user("wasm memory limit exceeds usize"))?;
        let limits = StoreLimitsBuilder::new()
            .memory_size(max_memory_bytes)
            .build();
        let mut store = Store::new(&engine, limits);
        store.limiter(|state| state);
        if config.max_fuel > 0 {
            store
                .set_fuel(config.max_fuel)
                .map_err(CocoError::compute)?;
        }

        let linker = Linker::new(&engine);
        let instance = linker
            .instantiate(&mut store, &module)
            .map_err(CocoError::compute)?;

        let mut export_names = Vec::new();
        let mut language_export = None;
        for export in module.exports() {
            let name = export.name().to_string();
            let is_func = matches!(export.ty(), ExternType::Func(_));
            if config
                .language_export
                .as_deref()
                .map(|expected| expected == name.as_str() && is_func)
                .unwrap_or(false)
            {
                language_export = Some(name.clone());
            }
            if language_export.is_none() && is_func && name.starts_with("tree_sitter_") {
                language_export = Some(name.clone());
            }
            export_names.push(name);
        }

        let language_export = match (config.language_export, language_export) {
            (Some(expected), Some(found)) if expected == found => found,
            (Some(expected), _) => {
                return Err(CocoError::compute(format!(
                    "wasm grammar missing expected export: {expected}"
                )));
            }
            (None, Some(found)) => found,
            (None, None) => {
                return Err(CocoError::compute(
                    "wasm grammar missing tree_sitter_* export",
                ));
            }
        };

        Ok(Self {
            engine,
            module,
            store,
            instance,
            export_names,
            language_export,
        })
    }

    /// Returns the export names declared by the module.
    pub fn export_names(&self) -> &[String] {
        &self.export_names
    }

    /// Returns the selected language export name.
    pub fn language_export(&self) -> &str {
        self.language_export.as_str()
    }

    /// Returns a reference to the compiled module.
    pub fn module(&self) -> &Module {
        &self.module
    }

    /// Returns a mutable reference to the store.
    pub fn store(&mut self) -> &mut Store<wasmtime::StoreLimits> {
        &mut self.store
    }

    /// Returns a reference to the instantiated module.
    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    /// Returns a reference to the engine backing the module.
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}
