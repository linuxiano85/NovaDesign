use crate::{new_entity_id, Discipline, EntityId};
use serde::{Deserialize, Serialize};

/// Layer for organizing drawing elements by discipline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: EntityId,
    pub name: String,
    pub discipline: Discipline,
    pub visible: bool,
    pub locked: bool,
    pub color: Color,
    pub line_weight: f64,
}

impl Layer {
    pub fn new(name: String, discipline: Discipline) -> Self {
        Self {
            id: new_entity_id(),
            name,
            discipline,
            visible: true,
            locked: false,
            color: Color::default_for_discipline(discipline),
            line_weight: 1.0,
        }
    }
}

/// Color representation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// Get default color for discipline
    pub fn default_for_discipline(discipline: Discipline) -> Self {
        match discipline {
            Discipline::Architecture => Self::rgb(0, 0, 0), // Black
            Discipline::Electrical => Self::rgb(255, 0, 0), // Red
            Discipline::Plumbing => Self::rgb(0, 0, 255),   // Blue
            Discipline::Masonry => Self::rgb(139, 69, 19),  // Brown
            Discipline::Drywall => Self::rgb(255, 165, 0),  // Orange
            Discipline::Painting => Self::rgb(255, 255, 0), // Yellow
            Discipline::SuspendedCeiling => Self::rgb(128, 0, 128), // Purple
        }
    }
}

/// Layer manager for organizing project layers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerManager {
    pub layers: Vec<Layer>,
}

impl LayerManager {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn get_layer_by_discipline(&self, discipline: Discipline) -> Option<&Layer> {
        self.layers
            .iter()
            .find(|layer| layer.discipline == discipline)
    }

    pub fn set_layer_visibility(&mut self, layer_id: EntityId, visible: bool) {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == layer_id) {
            layer.visible = visible;
        }
    }

    /// Create default layers for all disciplines
    pub fn create_default_layers(&mut self) {
        use strum::IntoEnumIterator;

        for discipline in Discipline::iter() {
            let layer_name = match discipline {
                Discipline::Architecture => "Architecture",
                Discipline::Electrical => "Electrical",
                Discipline::Plumbing => "Plumbing",
                Discipline::Masonry => "Masonry",
                Discipline::Drywall => "Drywall",
                Discipline::Painting => "Painting",
                Discipline::SuspendedCeiling => "Suspended Ceiling",
            };

            self.add_layer(Layer::new(layer_name.to_string(), discipline));
        }
    }
}

impl Default for LayerManager {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.create_default_layers();
        manager
    }
}
