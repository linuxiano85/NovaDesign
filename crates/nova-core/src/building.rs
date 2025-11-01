use crate::{new_entity_id, Component, Discipline, EntityId, Point2D};
use serde::{Deserialize, Serialize};

/// Project represents the root container for all design data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub buildings: Vec<Building>,
    pub created_at: String, // ISO 8601 timestamp
    pub modified_at: String,
    pub version: String,
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: new_entity_id(),
            name,
            description,
            buildings: Vec::new(),
            created_at: now.clone(),
            modified_at: now,
            version: "0.1.0".to_string(),
        }
    }

    pub fn add_building(&mut self, building: Building) {
        self.buildings.push(building);
        self.update_modified();
    }

    fn update_modified(&mut self) {
        self.modified_at = chrono::Utc::now().to_rfc3339();
    }
}

/// Building contains multiple floors/levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: EntityId,
    pub name: String,
    pub address: String,
    pub floors: Vec<Floor>,
}

impl Building {
    pub fn new(name: String, address: String) -> Self {
        Self {
            id: new_entity_id(),
            name,
            address,
            floors: Vec::new(),
        }
    }

    pub fn add_floor(&mut self, floor: Floor) {
        self.floors.push(floor);
    }
}

/// Floor/Level (piano/livello) represents a single level of a building
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floor {
    pub id: EntityId,
    pub name: String,
    pub level: f64,          // elevation in meters
    pub ceiling_height: f64, // height in meters
    pub walls: Vec<Wall>,
    pub openings: Vec<Opening>,
    pub components: Vec<Component>,
}

impl Floor {
    pub fn new(name: String, level: f64, ceiling_height: f64) -> Self {
        Self {
            id: new_entity_id(),
            name,
            level,
            ceiling_height,
            walls: Vec::new(),
            openings: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn add_wall(&mut self, wall: Wall) {
        self.walls.push(wall);
    }

    pub fn add_opening(&mut self, opening: Opening) {
        self.openings.push(opening);
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    /// Get walls filtered by discipline
    pub fn walls_by_discipline(&self, discipline: Discipline) -> Vec<&Wall> {
        self.walls
            .iter()
            .filter(|wall| wall.disciplines.contains(&discipline))
            .collect()
    }
}

/// Wall represents a structural or partition wall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wall {
    pub id: EntityId,
    pub name: String,
    pub start_point: Point2D,
    pub end_point: Point2D,
    pub thickness: f64, // in meters
    pub height: f64,    // in meters
    pub material: String,
    pub disciplines: Vec<Discipline>,
}

impl Wall {
    pub fn new(name: String, start: Point2D, end: Point2D, thickness: f64, height: f64) -> Self {
        Self {
            id: new_entity_id(),
            name,
            start_point: start,
            end_point: end,
            thickness,
            height,
            material: "Generic".to_string(),
            disciplines: vec![Discipline::Architecture],
        }
    }

    pub fn length(&self) -> f64 {
        let dx = self.end_point.x - self.start_point.x;
        let dy = self.end_point.y - self.start_point.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn area(&self) -> f64 {
        self.length() * self.height
    }
}

/// Opening represents doors, windows, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opening {
    pub id: EntityId,
    pub name: String,
    pub opening_type: OpeningType,
    pub position: Point2D,
    pub width: f64,
    pub height: f64,
    pub wall_id: Option<EntityId>, // Associated wall
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpeningType {
    Door,
    Window,
    Opening,
}

impl Opening {
    pub fn new(
        name: String,
        opening_type: OpeningType,
        position: Point2D,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            id: new_entity_id(),
            name,
            opening_type,
            position,
            width,
            height,
            wall_id: None,
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}
