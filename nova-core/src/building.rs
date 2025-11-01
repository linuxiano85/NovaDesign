use crate::{DesignElement, GeometryProperties, Phase};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Building project containing all elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub levels: Vec<Level>,
    pub walls: Vec<Wall>,
}

impl Building {
    pub fn new(name: String, address: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            address,
            levels: Vec::new(),
            walls: Vec::new(),
        }
    }

    pub fn add_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    pub fn add_wall(&mut self, wall: Wall) {
        self.walls.push(wall);
    }
}

/// Building level/floor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: Uuid,
    pub name: String,
    pub elevation: f64, // meters above ground
    pub height: f64,    // level height in meters
    pub phase: Phase,
}

impl Level {
    pub fn new(name: String, elevation: f64, height: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            elevation,
            height,
            phase: Phase::Nuovo,
        }
    }
}

impl DesignElement for Level {
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

/// Wall element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wall {
    pub id: Uuid,
    pub name: String,
    pub geometry: GeometryProperties,
    pub length: f64,    // meters
    pub height: f64,    // meters
    pub thickness: f64, // meters
    pub material: String,
    pub phase: Phase,
}

impl Wall {
    pub fn new(name: String, length: f64, height: f64, thickness: f64, material: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            geometry: GeometryProperties::default(),
            length,
            height,
            thickness,
            material,
            phase: Phase::Nuovo,
        }
    }

    pub fn area(&self) -> f64 {
        self.length * self.height
    }

    pub fn volume(&self) -> f64 {
        self.length * self.height * self.thickness
    }
}

impl DesignElement for Wall {
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
