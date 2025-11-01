use serde::{Deserialize, Serialize};

/// Plugin trait that all plugins must implement
pub trait Plugin {
    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Plugin description
    fn description(&self) -> &str;

    /// Initialize the plugin
    fn initialize(&mut self) -> Result<(), PluginError>;

    /// Execute plugin functionality
    fn execute(&self, input: &PluginInput) -> Result<PluginOutput, PluginError>;

    /// Cleanup resources
    fn cleanup(&mut self) -> Result<(), PluginError>;
}

/// Plugin input data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInput {
    pub command: String,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Plugin output data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginOutput {
    pub success: bool,
    pub data: serde_json::Value,
    pub message: Option<String>,
}

/// Plugin error types
#[derive(Debug, Clone)]
pub enum PluginError {
    InitializationFailed(String),
    ExecutionFailed(String),
    SerializationError(String),
    InvalidInput(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::InitializationFailed(msg) => {
                write!(f, "Plugin initialization failed: {}", msg)
            }
            PluginError::ExecutionFailed(msg) => write!(f, "Plugin execution failed: {}", msg),
            PluginError::SerializationError(msg) => {
                write!(f, "Plugin serialization error: {}", msg)
            }
            PluginError::InvalidInput(msg) => write!(f, "Invalid plugin input: {}", msg),
        }
    }
}

impl std::error::Error for PluginError {}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub wasm_path: String,
}

/// Plugin registry for managing loaded plugins
pub struct PluginRegistry {
    plugins: std::collections::HashMap<String, PluginMetadata>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: std::collections::HashMap::new(),
        }
    }

    /// Register a new plugin
    pub fn register(&mut self, metadata: PluginMetadata) {
        self.plugins.insert(metadata.name.clone(), metadata);
    }

    /// Get plugin metadata by name
    pub fn get(&self, name: &str) -> Option<&PluginMetadata> {
        self.plugins.get(name)
    }

    /// List all registered plugins
    pub fn list(&self) -> Vec<&PluginMetadata> {
        self.plugins.values().collect()
    }

    /// Remove a plugin
    pub fn unregister(&mut self, name: &str) -> Option<PluginMetadata> {
        self.plugins.remove(name)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Sample plugin implementation for demonstration
pub struct SamplePlugin {
    name: String,
    version: String,
    description: String,
    initialized: bool,
}

impl SamplePlugin {
    pub fn new() -> Self {
        Self {
            name: "SamplePlugin".to_string(),
            version: "0.1.0".to_string(),
            description: "A sample plugin for demonstration".to_string(),
            initialized: false,
        }
    }
}

impl Default for SamplePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for SamplePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        self.initialized = true;
        Ok(())
    }

    fn execute(&self, input: &PluginInput) -> Result<PluginOutput, PluginError> {
        if !self.initialized {
            return Err(PluginError::ExecutionFailed(
                "Plugin not initialized".to_string(),
            ));
        }

        match input.command.as_str() {
            "hello" => Ok(PluginOutput {
                success: true,
                data: serde_json::json!({"message": "Hello from SamplePlugin!"}),
                message: Some("Command executed successfully".to_string()),
            }),
            "echo" => {
                let message = input
                    .parameters
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("No message provided");

                Ok(PluginOutput {
                    success: true,
                    data: serde_json::json!({"echo": message}),
                    message: Some("Echo command executed".to_string()),
                })
            }
            _ => Err(PluginError::InvalidInput(format!(
                "Unknown command: {}",
                input.command
            ))),
        }
    }

    fn cleanup(&mut self) -> Result<(), PluginError> {
        self.initialized = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();

        let metadata = PluginMetadata {
            name: "TestPlugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            license: "GPL-3.0".to_string(),
            wasm_path: "/path/to/plugin.wasm".to_string(),
        };

        registry.register(metadata.clone());

        assert!(registry.get("TestPlugin").is_some());
        assert_eq!(registry.list().len(), 1);

        let removed = registry.unregister("TestPlugin");
        assert!(removed.is_some());
        assert_eq!(registry.list().len(), 0);
    }

    #[test]
    fn test_sample_plugin() {
        let mut plugin = SamplePlugin::new();

        assert_eq!(plugin.name(), "SamplePlugin");
        assert_eq!(plugin.version(), "0.1.0");

        // Test initialization
        assert!(plugin.initialize().is_ok());

        // Test hello command
        let input = PluginInput {
            command: "hello".to_string(),
            parameters: std::collections::HashMap::new(),
        };

        let result = plugin.execute(&input);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.success);

        // Test echo command
        let mut params = std::collections::HashMap::new();
        params.insert("message".to_string(), serde_json::json!("test message"));

        let input = PluginInput {
            command: "echo".to_string(),
            parameters: params,
        };

        let result = plugin.execute(&input);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["echo"], "test message");

        // Test cleanup
        assert!(plugin.cleanup().is_ok());
    }

    #[test]
    fn test_invalid_command() {
        let mut plugin = SamplePlugin::new();
        plugin.initialize().unwrap();

        let input = PluginInput {
            command: "invalid".to_string(),
            parameters: std::collections::HashMap::new(),
        };

        let result = plugin.execute(&input);
        assert!(result.is_err());
    }
}
