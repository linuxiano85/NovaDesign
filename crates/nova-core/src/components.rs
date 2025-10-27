use crate::{new_entity_id, Discipline, EntityId, Phase, Point2D};
use serde::{Deserialize, Serialize};

/// Component represents MEP components (electrical, plumbing, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: EntityId,
    pub name: String,
    pub component_type: ComponentType,
    pub position: Point2D,
    pub rotation: f64, // degrees
    pub discipline: Discipline,
    pub phase: Phase,
    pub properties: ComponentProperties,
}

impl Component {
    pub fn new(
        name: String,
        component_type: ComponentType,
        position: Point2D,
        discipline: Discipline,
    ) -> Self {
        Self {
            id: new_entity_id(),
            name,
            component_type,
            position,
            rotation: 0.0,
            discipline,
            phase: Phase::New,
            properties: ComponentProperties::default(),
        }
    }
}

/// Types of components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    // Electrical components
    Electrical(ElectricalComponent),
    // Plumbing components
    Plumbing(PlumbingComponent),
    // Drywall components
    Drywall(DrywallComponent),
    // Suspended ceiling components
    SuspendedCeiling(SuspendedCeilingComponent),
}

/// Electrical component types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ElectricalComponent {
    Outlet,
    Switch,
    Light,
    Panel,
    Junction,
    Conduit,
}

/// Plumbing component types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PlumbingComponent {
    Sink,
    Toilet,
    Shower,
    Pipe,
    Valve,
    Fixture,
}

/// Drywall component types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DrywallComponent {
    Stud,
    Track,
    Panel,
    Corner,
    Joint,
}

/// Suspended ceiling component types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SuspendedCeilingComponent {
    Tile,
    Grid,
    Hanger,
    Support,
    Light,
}

/// Component properties for customization
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComponentProperties {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub size: Option<String>,
    pub voltage: Option<f64>,       // for electrical
    pub amperage: Option<f64>,      // for electrical
    pub pipe_diameter: Option<f64>, // for plumbing
    pub material: Option<String>,
    pub custom_properties: std::collections::HashMap<String, String>,
}



/// Symbol data for plan view representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: EntityId,
    pub name: String,
    pub discipline: Discipline,
    pub component_type: String,
    pub svg_data: String, // SVG content for rendering
    pub width: f64,
    pub height: f64,
}

impl Symbol {
    pub fn new(
        name: String,
        discipline: Discipline,
        component_type: String,
        svg_data: String,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            id: new_entity_id(),
            name,
            discipline,
            component_type,
            svg_data,
            width,
            height,
        }
    }
}

/// Symbol library for managing component symbols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolLibrary {
    pub symbols: Vec<Symbol>,
}

impl SymbolLibrary {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn get_symbol(&self, discipline: Discipline, component_type: &str) -> Option<&Symbol> {
        self.symbols
            .iter()
            .find(|s| s.discipline == discipline && s.component_type == component_type)
    }

    /// Load default electrical and plumbing symbols
    pub fn load_defaults(&mut self) {
        // Electrical symbols - minimal placeholders
        self.add_symbol(Symbol::new(
            "Outlet".to_string(),
            Discipline::Electrical,
            "Outlet".to_string(),
            "<svg><circle cx='12' cy='12' r='10' fill='none' stroke='black' stroke-width='2'/><circle cx='8' cy='8' r='1' fill='black'/><circle cx='16' cy='8' r='1' fill='black'/></svg>".to_string(),
            24.0,
            24.0,
        ));

        self.add_symbol(Symbol::new(
            "Switch".to_string(),
            Discipline::Electrical,
            "Switch".to_string(),
            "<svg><rect x='2' y='10' width='20' height='4' fill='none' stroke='black' stroke-width='2'/><line x1='8' y1='10' x2='16' y2='6' stroke='black' stroke-width='2'/></svg>".to_string(),
            24.0,
            24.0,
        ));

        self.add_symbol(Symbol::new(
            "Light".to_string(),
            Discipline::Electrical,
            "Light".to_string(),
            "<svg><circle cx='12' cy='12' r='10' fill='none' stroke='black' stroke-width='2'/><path d='M6,6 L18,18 M18,6 L6,18' stroke='black' stroke-width='2'/></svg>".to_string(),
            24.0,
            24.0,
        ));

        // Plumbing symbols - minimal placeholders
        self.add_symbol(Symbol::new(
            "Sink".to_string(),
            Discipline::Plumbing,
            "Sink".to_string(),
            "<svg><rect x='4' y='8' width='16' height='10' fill='none' stroke='blue' stroke-width='2' rx='2'/><circle cx='8' cy='13' r='2' fill='none' stroke='blue'/><circle cx='16' cy='13' r='2' fill='none' stroke='blue'/></svg>".to_string(),
            24.0,
            24.0,
        ));

        self.add_symbol(Symbol::new(
            "Toilet".to_string(),
            Discipline::Plumbing,
            "Toilet".to_string(),
            "<svg><ellipse cx='12' cy='12' rx='8' ry='6' fill='none' stroke='blue' stroke-width='2'/><rect x='10' y='6' width='4' height='4' fill='none' stroke='blue' stroke-width='2'/></svg>".to_string(),
            24.0,
            24.0,
        ));
    }
}

impl Default for SymbolLibrary {
    fn default() -> Self {
        let mut library = Self::new();
        library.load_defaults();
        library
    }
}
