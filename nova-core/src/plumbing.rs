use crate::{DesignElement, GeometryProperties, Phase};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of plumbing devices
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlumbingDeviceType {
    Sink,     // Lavandino
    Toilet,   // WC
    Shower,   // Doccia
    Bathtub,  // Vasca
    Radiator, // Radiatore
    Pipe,     // Tubazione
    Valve,    // Valvola
    Faucet,   // Rubinetto
}

impl PlumbingDeviceType {
    pub fn name_it(&self) -> &'static str {
        match self {
            PlumbingDeviceType::Sink => "Lavandino",
            PlumbingDeviceType::Toilet => "WC",
            PlumbingDeviceType::Shower => "Doccia",
            PlumbingDeviceType::Bathtub => "Vasca",
            PlumbingDeviceType::Radiator => "Radiatore",
            PlumbingDeviceType::Pipe => "Tubazione",
            PlumbingDeviceType::Valve => "Valvola",
            PlumbingDeviceType::Faucet => "Rubinetto",
        }
    }
}

/// Plumbing device in the building
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlumbingDevice {
    pub id: Uuid,
    pub name: String,
    pub device_type: PlumbingDeviceType,
    pub geometry: GeometryProperties,
    pub diameter: Option<f64>,  // mm for pipes
    pub flow_rate: Option<f64>, // L/min
    pub phase: Phase,
}

impl PlumbingDevice {
    pub fn new(name: String, device_type: PlumbingDeviceType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            device_type,
            geometry: GeometryProperties::default(),
            diameter: None,
            flow_rate: None,
            phase: Phase::Nuovo,
        }
    }

    pub fn with_diameter(mut self, diameter_mm: f64) -> Self {
        self.diameter = Some(diameter_mm);
        self
    }

    pub fn with_flow_rate(mut self, flow_rate_lmin: f64) -> Self {
        self.flow_rate = Some(flow_rate_lmin);
        self
    }
}

impl DesignElement for PlumbingDevice {
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
