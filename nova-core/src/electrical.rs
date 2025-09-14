use crate::{DesignElement, GeometryProperties, Phase};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of electrical devices
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricalDeviceType {
    Outlet,   // Presa
    Switch,   // Interruttore
    Light,    // Illuminazione
    Panel,    // Quadro elettrico
    Junction, // Scatola di derivazione
    Conduit,  // Tubo corrugato
}

impl ElectricalDeviceType {
    pub fn name_it(&self) -> &'static str {
        match self {
            ElectricalDeviceType::Outlet => "Presa",
            ElectricalDeviceType::Switch => "Interruttore",
            ElectricalDeviceType::Light => "Illuminazione",
            ElectricalDeviceType::Panel => "Quadro elettrico",
            ElectricalDeviceType::Junction => "Scatola di derivazione",
            ElectricalDeviceType::Conduit => "Tubo corrugato",
        }
    }
}

/// Electrical device in the building
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalDevice {
    pub id: Uuid,
    pub name: String,
    pub device_type: ElectricalDeviceType,
    pub geometry: GeometryProperties,
    pub power_rating: Option<f64>, // watts
    pub circuit: Option<String>,
    pub phase: Phase,
}

impl ElectricalDevice {
    pub fn new(name: String, device_type: ElectricalDeviceType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            device_type,
            geometry: GeometryProperties::default(),
            power_rating: None,
            circuit: None,
            phase: Phase::Nuovo,
        }
    }

    pub fn with_power(mut self, watts: f64) -> Self {
        self.power_rating = Some(watts);
        self
    }

    pub fn with_circuit(mut self, circuit: String) -> Self {
        self.circuit = Some(circuit);
        self
    }
}

impl DesignElement for ElectricalDevice {
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
