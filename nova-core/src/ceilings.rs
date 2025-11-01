use crate::{DesignElement, GeometryProperties, Phase};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of ceiling profiles
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CeilingProfileType {
    MainT,      // Profilo T principale (24mm)
    SecondaryT, // Profilo T secondario (24mm)
    PerimeterL, // Angolare perimetrale (19mm)
    Hanger,     // Pendinatura
}

impl CeilingProfileType {
    pub fn name_it(&self) -> &'static str {
        match self {
            CeilingProfileType::MainT => "Profilo T principale",
            CeilingProfileType::SecondaryT => "Profilo T secondario",
            CeilingProfileType::PerimeterL => "Angolare perimetrale",
            CeilingProfileType::Hanger => "Pendinatura",
        }
    }
}

/// Suspended ceiling system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendedCeiling {
    pub id: Uuid,
    pub name: String,
    pub geometry: GeometryProperties,
    pub length: f64,            // meters
    pub width: f64,             // meters
    pub height: f64,            // suspension height in meters
    pub main_spacing: f64,      // spacing of main T profiles (typically 1.2m)
    pub secondary_spacing: f64, // spacing of secondary T profiles (typically 0.6m)
    pub tile_size: f64,         // ceiling tile size (typically 0.6m)
    pub phase: Phase,
}

impl SuspendedCeiling {
    pub fn new(name: String, length: f64, width: f64, height: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            geometry: GeometryProperties::default(),
            length,
            width,
            height,
            main_spacing: 1.2,      // 120cm spacing for main T
            secondary_spacing: 0.6, // 60cm spacing for secondary T
            tile_size: 0.6,         // 60x60cm tiles
            phase: Phase::Nuovo,
        }
    }

    pub fn area(&self) -> f64 {
        self.length * self.width
    }

    /// Calculate linear meters of main T profiles needed
    pub fn main_t_length(&self) -> f64 {
        let num_mains = (self.width / self.main_spacing).ceil();
        num_mains * self.length
    }

    /// Calculate linear meters of secondary T profiles needed
    pub fn secondary_t_length(&self) -> f64 {
        let num_secondaries = (self.length / self.secondary_spacing).ceil();
        num_secondaries * self.width
    }

    /// Calculate linear meters of perimeter angle needed
    pub fn perimeter_angle_length(&self) -> f64 {
        2.0 * (self.length + self.width)
    }

    /// Calculate number of hangers needed (1 every 1.2m on main profiles)
    pub fn hanger_count(&self) -> u32 {
        let main_profiles = (self.width / self.main_spacing).ceil() as u32;
        let hangers_per_profile = (self.length / 1.2).ceil() as u32;
        main_profiles * hangers_per_profile
    }

    /// Calculate number of ceiling tiles needed
    pub fn tile_count(&self) -> u32 {
        let tiles_length = (self.length / self.tile_size).ceil() as u32;
        let tiles_width = (self.width / self.tile_size).ceil() as u32;
        tiles_length * tiles_width
    }
}

impl DesignElement for SuspendedCeiling {
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
