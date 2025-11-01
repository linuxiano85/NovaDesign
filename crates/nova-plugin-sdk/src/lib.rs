use nova_core::{Discipline, EntityId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod manager;
pub mod traits;
pub mod wasm_host;

pub use manager::*;
pub use traits::*;
// pub use wasm_host::*; // Commented out to avoid ambiguous re-exports
pub use wasm_host::WasmHost;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub supported_disciplines: Vec<Discipline>,
    pub plugin_type: PluginType,
    pub wasm_module: Option<String>, // Path to WASM module
}

/// Types of plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginType {
    /// BOM calculation plugin
    BomCalculator,
    /// Import/Export plugin
    ImportExport,
    /// Analysis plugin
    Analysis,
    /// Rendering plugin
    Renderer,
    /// Custom tool plugin
    Tool,
}

/// Plugin execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    pub project_id: EntityId,
    pub selected_entities: Vec<EntityId>,
    pub current_discipline: Option<Discipline>,
    pub properties: HashMap<String, String>,
}

/// Plugin execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub modified_entities: Vec<EntityId>,
}

/// Plugin interface for host communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRequest {
    pub action: String,
    pub context: PluginContext,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResponse {
    pub result: PluginResult,
    pub logs: Vec<String>,
}

/// Plugin registry entry
#[derive(Debug, Clone)]
pub struct PluginRegistryEntry {
    pub metadata: PluginMetadata,
    pub is_enabled: bool,
    pub is_loaded: bool,
}

impl PluginRegistryEntry {
    pub fn new(metadata: PluginMetadata) -> Self {
        Self {
            metadata,
            is_enabled: true,
            is_loaded: false,
        }
    }
}

/// Plugin errors
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {name}")]
    PluginNotFound { name: String },
    #[error("Plugin load error: {message}")]
    LoadError { message: String },
    #[error("Plugin execution error: {message}")]
    ExecutionError { message: String },
    #[error("WASM runtime error: {0}")]
    WasmError(#[from] anyhow::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PluginError>;
