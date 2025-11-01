use crate::{DesignElement, GeometryProperties, Phase};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of drywall elements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrywallElementType {
    Stud,   // Montante
    Track,  // Guida
    Board,  // Lastra
    Fixing, // Tassello/Fissaggio
}

impl DrywallElementType {
    pub fn name_it(&self) -> &'static str {
        match self {
            DrywallElementType::Stud => "Montante",
            DrywallElementType::Track => "Guida",
            DrywallElementType::Board => "Lastra",
            DrywallElementType::Fixing => "Tassello",
        }
    }
}

/// Substrate types for drywall fixings
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubstrateType {
    Concrete, // Calcestruzzo
    Brick,    // Mattone
    Hollow,   // Forato
    Wood,     // Legno
    Steel,    // Acciaio
}

impl SubstrateType {
    pub fn name_it(&self) -> &'static str {
        match self {
            SubstrateType::Concrete => "Calcestruzzo",
            SubstrateType::Brick => "Mattone",
            SubstrateType::Hollow => "Forato",
            SubstrateType::Wood => "Legno",
            SubstrateType::Steel => "Acciaio",
        }
    }
}

/// Drywall wall system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrywallWall {
    pub id: Uuid,
    pub name: String,
    pub geometry: GeometryProperties,
    pub length: f64,          // meters
    pub height: f64,          // meters
    pub thickness: f64,       // mm
    pub stud_spacing: f64,    // mm (typically 400 or 600)
    pub board_thickness: f64, // mm (typically 12.5)
    pub substrate_type: SubstrateType,
    pub phase: Phase,
}

impl DrywallWall {
    pub fn new(name: String, length: f64, height: f64, thickness: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            geometry: GeometryProperties::default(),
            length,
            height,
            thickness,
            stud_spacing: 600.0,   // default 60cm spacing
            board_thickness: 12.5, // default 12.5mm board
            substrate_type: SubstrateType::Concrete,
            phase: Phase::Nuovo,
        }
    }

    pub fn area(&self) -> f64 {
        self.length * self.height
    }

    /// Calculate number of studs needed
    pub fn stud_count(&self) -> u32 {
        let num_spaces = (self.length * 1000.0 / self.stud_spacing).ceil() as u32;
        num_spaces + 1 // add one for the end stud
    }

    /// Calculate linear meters of track needed (top and bottom)
    pub fn track_length(&self) -> f64 {
        self.length * 2.0 // top and bottom tracks
    }

    /// Calculate square meters of boards needed (both sides)
    pub fn board_area(&self) -> f64 {
        self.area() * 2.0 // both sides of wall
    }
}

impl DesignElement for DrywallWall {
    fn id(&self) -> Uuid {
        self.id
    }

    fn phase(&self) -> &Phase {
        &self.phase
    }

    fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
    }
}
