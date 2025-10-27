use crate::{PluginContext, PluginResult, Result};
use nova_core::{Discipline, Project};

/// Base trait for all plugins
pub trait Plugin: Send + Sync {
    /// Get plugin name
    fn name(&self) -> &str;

    /// Get plugin version
    fn version(&self) -> &str;

    /// Get supported disciplines
    fn supported_disciplines(&self) -> Vec<Discipline>;

    /// Initialize the plugin
    fn initialize(&mut self) -> Result<()>;

    /// Cleanup the plugin
    fn cleanup(&mut self) -> Result<()>;

    /// Check if plugin can handle the given context
    fn can_handle(&self, context: &PluginContext) -> bool;
}

/// Trait for BOM calculation plugins
pub trait BomCalculatorPlugin: Plugin {
    /// Calculate BOM for the given project
    fn calculate_bom(&self, project: &Project, context: &PluginContext) -> Result<PluginResult>;
}

/// Trait for import/export plugins
pub trait ImportExportPlugin: Plugin {
    /// Get supported file extensions
    fn supported_extensions(&self) -> Vec<String>;

    /// Import data from file
    fn import(&self, file_path: &str, context: &PluginContext) -> Result<PluginResult>;

    /// Export data to file
    fn export(
        &self,
        project: &Project,
        file_path: &str,
        context: &PluginContext,
    ) -> Result<PluginResult>;
}

/// Trait for analysis plugins
pub trait AnalysisPlugin: Plugin {
    /// Perform analysis on the project
    fn analyze(&self, project: &Project, context: &PluginContext) -> Result<PluginResult>;

    /// Get analysis parameters
    fn get_parameters(&self) -> Vec<AnalysisParameter>;
}

/// Analysis parameter definition
#[derive(Debug, Clone)]
pub struct AnalysisParameter {
    pub name: String,
    pub parameter_type: ParameterType,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
}

/// Parameter types for analysis
#[derive(Debug, Clone)]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Choice(Vec<String>),
}

/// Trait for rendering plugins
pub trait RenderingPlugin: Plugin {
    /// Render project view
    fn render(&self, project: &Project, context: &PluginContext) -> Result<PluginResult>;

    /// Get rendering options
    fn get_render_options(&self) -> Vec<RenderOption>;
}

/// Rendering option definition
#[derive(Debug, Clone)]
pub struct RenderOption {
    pub name: String,
    pub option_type: ParameterType,
    pub description: String,
    pub default_value: Option<String>,
}

/// Trait for tool plugins
pub trait ToolPlugin: Plugin {
    /// Execute the tool
    fn execute(&self, project: &mut Project, context: &PluginContext) -> Result<PluginResult>;

    /// Get tool UI configuration
    fn get_ui_config(&self) -> ToolUIConfig;
}

/// Tool UI configuration
#[derive(Debug, Clone)]
pub struct ToolUIConfig {
    pub icon: Option<String>,
    pub tooltip: String,
    pub menu_path: String,
    pub keyboard_shortcut: Option<String>,
    pub toolbar_position: Option<u32>,
}

/// Marker trait for WASM plugins
pub trait WasmPlugin {
    /// Get the WASM module bytes
    fn get_wasm_module(&self) -> &[u8];

    /// Get the plugin entry point function name
    fn entry_point(&self) -> &str {
        "execute"
    }
}
