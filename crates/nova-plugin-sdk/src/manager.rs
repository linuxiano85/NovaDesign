use crate::{
    PluginError, PluginMetadata, PluginRegistryEntry, PluginRequest, PluginResponse, Result,
};
use std::collections::HashMap;
use std::path::PathBuf;

/// Plugin manager for loading and managing plugins
pub struct PluginManager {
    plugins: HashMap<String, PluginRegistryEntry>,
    plugin_directory: PathBuf,
}

impl PluginManager {
    pub fn new(plugin_directory: PathBuf) -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_directory,
        }
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, metadata: PluginMetadata) -> Result<()> {
        let entry = PluginRegistryEntry::new(metadata.clone());
        self.plugins.insert(metadata.name.clone(), entry);
        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister_plugin(&mut self, plugin_name: &str) -> Result<()> {
        self.plugins.remove(plugin_name);
        Ok(())
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, plugin_name: &str) -> Result<()> {
        if let Some(entry) = self.plugins.get_mut(plugin_name) {
            entry.is_enabled = true;
            Ok(())
        } else {
            Err(PluginError::PluginNotFound {
                name: plugin_name.to_string(),
            })
        }
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, plugin_name: &str) -> Result<()> {
        if let Some(entry) = self.plugins.get_mut(plugin_name) {
            entry.is_enabled = false;
            entry.is_loaded = false;
            Ok(())
        } else {
            Err(PluginError::PluginNotFound {
                name: plugin_name.to_string(),
            })
        }
    }

    /// Load a plugin (for WASM plugins, this means instantiating the module)
    pub fn load_plugin(&mut self, plugin_name: &str) -> Result<()> {
        if let Some(entry) = self.plugins.get_mut(plugin_name) {
            if !entry.is_enabled {
                return Err(PluginError::LoadError {
                    message: "Plugin is disabled".to_string(),
                });
            }

            if entry.is_loaded {
                return Ok(()); // Already loaded
            }

            // For WASM plugins, we would load and instantiate the module here
            // This is a stub implementation
            if let Some(_wasm_path) = &entry.metadata.wasm_module {
                // TODO: Load WASM module using wasmtime
                entry.is_loaded = true;
            }

            Ok(())
        } else {
            Err(PluginError::PluginNotFound {
                name: plugin_name.to_string(),
            })
        }
    }

    /// Execute a plugin
    pub fn execute_plugin(
        &self,
        plugin_name: &str,
        _request: PluginRequest,
    ) -> Result<PluginResponse> {
        if let Some(entry) = self.plugins.get(plugin_name) {
            if !entry.is_enabled || !entry.is_loaded {
                return Err(PluginError::ExecutionError {
                    message: "Plugin is not enabled or loaded".to_string(),
                });
            }

            // For WASM plugins, this would call into the WASM module
            // This is a stub implementation
            let result = crate::PluginResult {
                success: true,
                message: "Plugin executed successfully".to_string(),
                data: None,
                modified_entities: Vec::new(),
            };

            Ok(PluginResponse {
                result,
                logs: vec!["Plugin executed".to_string()],
            })
        } else {
            Err(PluginError::PluginNotFound {
                name: plugin_name.to_string(),
            })
        }
    }

    /// Get list of registered plugins
    pub fn list_plugins(&self) -> Vec<&PluginRegistryEntry> {
        self.plugins.values().collect()
    }

    /// Get plugin by name
    pub fn get_plugin(&self, plugin_name: &str) -> Option<&PluginRegistryEntry> {
        self.plugins.get(plugin_name)
    }

    /// Scan plugin directory for new plugins
    pub fn scan_plugins(&mut self) -> Result<usize> {
        let mut loaded_count = 0;

        if !self.plugin_directory.exists() {
            return Ok(0);
        }

        // Scan for .toml metadata files and .wasm modules
        for entry in std::fs::read_dir(&self.plugin_directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                // Try to load plugin metadata
                if let Ok(metadata_content) = std::fs::read_to_string(&path) {
                    if let Ok(metadata) = toml::from_str::<PluginMetadata>(&metadata_content) {
                        self.register_plugin(metadata)?;
                        loaded_count += 1;
                    }
                }
            }
        }

        Ok(loaded_count)
    }

    /// Create a sample plugin metadata for development
    pub fn create_sample_plugin_metadata() -> PluginMetadata {
        PluginMetadata {
            name: "Sample BOM Calculator".to_string(),
            version: "0.1.0".to_string(),
            description: "A sample BOM calculation plugin".to_string(),
            author: "Nova Design Team".to_string(),
            license: "GPL-3.0-or-later".to_string(),
            supported_disciplines: vec![nova_core::Discipline::Drywall],
            plugin_type: crate::PluginType::BomCalculator,
            wasm_module: Some("sample_bom_calculator.wasm".to_string()),
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new(PathBuf::from("plugins"))
    }
}
