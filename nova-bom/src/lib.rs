use nova_core::{
    ceilings::SuspendedCeiling,
    drywall::{DrywallWall, SubstrateType},
    electrical::{ElectricalDevice, ElectricalDeviceType},
    DesignElement, Phase,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// BOM (Bill of Materials) item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomItem {
    pub code: String,
    pub description: String,
    pub unit: String,
    pub quantity: f64,
    pub category: String,
}

impl BomItem {
    pub fn new(
        code: String,
        description: String,
        unit: String,
        quantity: f64,
        category: String,
    ) -> Self {
        Self {
            code,
            description,
            unit,
            quantity,
            category,
        }
    }
}

/// BOM engine for calculating material quantities
pub struct BomEngine {
    items: Vec<BomItem>,
}

impl BomEngine {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add electrical devices to BOM
    pub fn add_electrical_devices(&mut self, devices: &[ElectricalDevice]) {
        let mut device_counts: HashMap<ElectricalDeviceType, u32> = HashMap::new();

        for device in devices {
            if device.phase() == &Phase::Nuovo {
                *device_counts.entry(device.device_type.clone()).or_insert(0) += 1;
            }
        }

        for (device_type, count) in device_counts {
            self.items.push(BomItem::new(
                format!("ELE_{:?}", device_type).to_uppercase(),
                device_type.name_it().to_string(),
                "pz".to_string(),
                count as f64,
                "Elettrico".to_string(),
            ));
        }
    }

    /// Add drywall materials to BOM
    pub fn add_drywall_materials(&mut self, walls: &[DrywallWall]) {
        let mut total_stud_count = 0u32;
        let mut total_track_length = 0.0f64;
        let mut total_board_area = 0.0f64;
        let mut fixings_by_substrate: HashMap<SubstrateType, u32> = HashMap::new();

        for wall in walls {
            if wall.phase() == &Phase::Nuovo {
                total_stud_count += wall.stud_count();
                total_track_length += wall.track_length();
                total_board_area += wall.board_area();

                // Assume 5 fixings per m² for boards
                let fixing_count = (wall.board_area() * 5.0) as u32;
                *fixings_by_substrate
                    .entry(wall.substrate_type.clone())
                    .or_insert(0) += fixing_count;
            }
        }

        if total_stud_count > 0 {
            self.items.push(BomItem::new(
                "DRY_STUD".to_string(),
                "Montante cartongesso".to_string(),
                "pz".to_string(),
                total_stud_count as f64,
                "Cartongesso".to_string(),
            ));
        }

        if total_track_length > 0.0 {
            self.items.push(BomItem::new(
                "DRY_TRACK".to_string(),
                "Guida cartongesso".to_string(),
                "m".to_string(),
                total_track_length,
                "Cartongesso".to_string(),
            ));
        }

        if total_board_area > 0.0 {
            self.items.push(BomItem::new(
                "DRY_BOARD".to_string(),
                "Lastra cartongesso".to_string(),
                "m²".to_string(),
                total_board_area,
                "Cartongesso".to_string(),
            ));
        }

        for (substrate_type, count) in fixings_by_substrate {
            self.items.push(BomItem::new(
                format!("DRY_FIX_{:?}", substrate_type).to_uppercase(),
                format!("Tasselli per {}", substrate_type.name_it()),
                "pz".to_string(),
                count as f64,
                "Cartongesso".to_string(),
            ));
        }
    }

    /// Add suspended ceiling materials to BOM
    pub fn add_suspended_ceilings(&mut self, ceilings: &[SuspendedCeiling]) {
        let mut total_main_t = 0.0f64;
        let mut total_secondary_t = 0.0f64;
        let mut total_perimeter_angle = 0.0f64;
        let mut total_hangers = 0u32;

        for ceiling in ceilings {
            if ceiling.phase() == &Phase::Nuovo {
                total_main_t += ceiling.main_t_length();
                total_secondary_t += ceiling.secondary_t_length();
                total_perimeter_angle += ceiling.perimeter_angle_length();
                total_hangers += ceiling.hanger_count();
            }
        }

        if total_main_t > 0.0 {
            self.items.push(BomItem::new(
                "CEIL_MAIN_T".to_string(),
                "Profilo T principale".to_string(),
                "m".to_string(),
                total_main_t,
                "Controsoffitti".to_string(),
            ));
        }

        if total_secondary_t > 0.0 {
            self.items.push(BomItem::new(
                "CEIL_SEC_T".to_string(),
                "Profilo T secondario".to_string(),
                "m".to_string(),
                total_secondary_t,
                "Controsoffitti".to_string(),
            ));
        }

        if total_perimeter_angle > 0.0 {
            self.items.push(BomItem::new(
                "CEIL_ANGLE".to_string(),
                "Angolare perimetrale".to_string(),
                "m".to_string(),
                total_perimeter_angle,
                "Controsoffitti".to_string(),
            ));
        }

        if total_hangers > 0 {
            self.items.push(BomItem::new(
                "CEIL_HANGER".to_string(),
                "Pendinatura".to_string(),
                "pz".to_string(),
                total_hangers as f64,
                "Controsoffitti".to_string(),
            ));
        }
    }

    /// Get all BOM items
    pub fn items(&self) -> &[BomItem] {
        &self.items
    }

    /// Export BOM to CSV format
    pub fn export_csv(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header
        wtr.write_record(["Codice", "Descrizione", "Unità", "Quantità", "Categoria"])?;

        // Write items
        for item in &self.items {
            wtr.write_record([
                &item.code,
                &item.description,
                &item.unit,
                &item.quantity.to_string(),
                &item.category,
            ])?;
        }

        let data = String::from_utf8(wtr.into_inner().map_err(|e| e.into_error())?).unwrap();
        Ok(data)
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for BomEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nova_core::electrical::ElectricalDevice;

    #[test]
    fn test_electrical_bom() {
        let mut engine = BomEngine::new();
        let devices = vec![
            ElectricalDevice::new("Presa 1".to_string(), ElectricalDeviceType::Outlet),
            ElectricalDevice::new("Presa 2".to_string(), ElectricalDeviceType::Outlet),
            ElectricalDevice::new("Interruttore 1".to_string(), ElectricalDeviceType::Switch),
        ];

        engine.add_electrical_devices(&devices);

        assert_eq!(engine.items().len(), 2); // 2 outlet types
        assert!(engine
            .items()
            .iter()
            .any(|item| item.description == "Presa" && item.quantity == 2.0));
        assert!(engine
            .items()
            .iter()
            .any(|item| item.description == "Interruttore" && item.quantity == 1.0));
    }

    #[test]
    fn test_drywall_bom() {
        let mut engine = BomEngine::new();
        let walls = vec![DrywallWall::new("Parete 1".to_string(), 3.0, 2.8, 100.0)];

        engine.add_drywall_materials(&walls);

        assert!(engine.items().len() >= 3); // studs, tracks, boards, fixings
        assert!(engine
            .items()
            .iter()
            .any(|item| item.description == "Montante cartongesso"));
        assert!(engine
            .items()
            .iter()
            .any(|item| item.description == "Guida cartongesso"));
        assert!(engine
            .items()
            .iter()
            .any(|item| item.description == "Lastra cartongesso"));
    }

    #[test]
    fn test_csv_export() {
        let mut engine = BomEngine::new();
        engine.items.push(BomItem::new(
            "TEST_001".to_string(),
            "Test Item".to_string(),
            "pz".to_string(),
            5.0,
            "Test".to_string(),
        ));

        let csv = engine.export_csv().unwrap();
        assert!(csv.contains("Codice,Descrizione,Unità,Quantità,Categoria"));
        assert!(csv.contains("TEST_001,Test Item,pz,5,Test"));
    }
}
