use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod building;
pub mod ceilings;
pub mod drywall;
pub mod electrical;
pub mod plumbing;

/// Phase of construction work
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// Existing elements
    Esistente,
    /// Elements to be demolished
    Demolizione,
    /// New elements
    Nuovo,
}

impl Phase {
    pub fn name(&self) -> &'static str {
        match self {
            Phase::Esistente => "Esistente",
            Phase::Demolizione => "Demolizione",
            Phase::Nuovo => "Nuovo",
        }
    }
}

/// Base trait for all design elements
pub trait DesignElement {
    fn id(&self) -> Uuid;
    fn phase(&self) -> &Phase;
    fn set_phase(&mut self, phase: Phase);
}

/// Common properties for geometric elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryProperties {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rotation: f64, // degrees
}

impl Default for GeometryProperties {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation: 0.0,
        }
    }
}
