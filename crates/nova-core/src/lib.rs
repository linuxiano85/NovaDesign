use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
use uuid::Uuid;

pub mod building;
pub mod components;
pub mod geometry;
pub mod layer;
pub mod material;

pub use building::*;
pub use components::*;
pub use geometry::*;
pub use layer::*;
pub use material::*;

/// Phases of construction work
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Phase {
    /// Existing conditions
    Existing,
    /// Demolition phase
    Demolition,
    /// New construction
    New,
}

/// Engineering disciplines
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Discipline {
    /// Architectural
    Architecture,
    /// Electrical systems
    Electrical,
    /// Plumbing/hydraulic systems
    Plumbing,
    /// Masonry work
    Masonry,
    /// Drywall (cartongesso)
    Drywall,
    /// Painting (imbiancino)
    Painting,
    /// Suspended ceilings (controsoffitti)
    SuspendedCeiling,
}

/// Unique identifier type for entities
pub type EntityId = Uuid;

/// Generate a new unique entity ID
pub fn new_entity_id() -> EntityId {
    Uuid::new_v4()
}

/// Error types for the core module
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Entity not found: {id}")]
    EntityNotFound { id: EntityId },
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CoreError>;
