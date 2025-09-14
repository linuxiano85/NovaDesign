use crate::{new_entity_id, Discipline, EntityId};
use serde::{Deserialize, Serialize};

/// Material definition for BOM calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub id: EntityId,
    pub name: String,
    pub code: String,
    pub unit: Unit,
    pub cost_per_unit: f64,
    pub density: Option<f64>, // kg/m³ or similar
    pub supplier: Option<String>,
    pub discipline: Discipline,
    pub category: MaterialCategory,
}

impl Material {
    pub fn new(
        name: String,
        code: String,
        unit: Unit,
        cost_per_unit: f64,
        discipline: Discipline,
        category: MaterialCategory,
    ) -> Self {
        Self {
            id: new_entity_id(),
            name,
            code,
            unit,
            cost_per_unit,
            density: None,
            supplier: None,
            discipline,
            category,
        }
    }
}

/// Units of measurement
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Unit {
    // Length units
    Meter,
    Centimeter,
    Millimeter,
    // Area units
    SquareMeter,
    // Volume units
    CubicMeter,
    Liter,
    // Count units
    Piece,
    Each,
    // Weight units
    Kilogram,
    Gram,
    // Electrical units
    MeterCable,
    Watt,
    Ampere,
}

/// Material categories for organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialCategory {
    // Drywall materials
    DrywallPanel,
    DrywallStud,
    DrywallTrack,
    DrywallScrew,
    DrywallCompound,
    DrywallTape,

    // Suspended ceiling materials
    CeilingTile,
    CeilingGrid,
    CeilingHanger,
    CeilingWire,

    // Electrical materials
    ElectricalCable,
    ElectricalConduit,
    ElectricalOutlet,
    ElectricalSwitch,
    ElectricalPanel,
    ElectricalFixture,

    // Plumbing materials
    PlumbingPipe,
    PlumbingFitting,
    PlumbingFixture,
    PlumbingValve,

    // Fixings and fasteners
    Anchor,
    Screw,
    Nail,
    Bolt,

    // Generic
    Other(String),
}

/// Material database for BOM calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialDatabase {
    pub materials: Vec<Material>,
}

impl MaterialDatabase {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
        }
    }

    pub fn add_material(&mut self, material: Material) {
        self.materials.push(material);
    }

    pub fn get_material_by_code(&self, code: &str) -> Option<&Material> {
        self.materials.iter().find(|m| m.code == code)
    }

    pub fn get_materials_by_category(&self, category: &MaterialCategory) -> Vec<&Material> {
        self.materials
            .iter()
            .filter(|m| std::mem::discriminant(&m.category) == std::mem::discriminant(category))
            .collect()
    }

    pub fn get_materials_by_discipline(&self, discipline: Discipline) -> Vec<&Material> {
        self.materials
            .iter()
            .filter(|m| m.discipline == discipline)
            .collect()
    }

    /// Load default materials for drywall and suspended ceiling calculations
    pub fn load_defaults(&mut self) {
        // Drywall materials
        self.add_material(Material::new(
            "Drywall Panel 12.5mm".to_string(),
            "DW_PANEL_125".to_string(),
            Unit::SquareMeter,
            15.50,
            Discipline::Drywall,
            MaterialCategory::DrywallPanel,
        ));

        self.add_material(Material::new(
            "Metal Stud 75mm".to_string(),
            "DW_STUD_75".to_string(),
            Unit::Meter,
            3.20,
            Discipline::Drywall,
            MaterialCategory::DrywallStud,
        ));

        self.add_material(Material::new(
            "Metal Track 75mm".to_string(),
            "DW_TRACK_75".to_string(),
            Unit::Meter,
            2.80,
            Discipline::Drywall,
            MaterialCategory::DrywallTrack,
        ));

        self.add_material(Material::new(
            "Drywall Screw 25mm".to_string(),
            "DW_SCREW_25".to_string(),
            Unit::Piece,
            0.05,
            Discipline::Drywall,
            MaterialCategory::DrywallScrew,
        ));

        // Suspended ceiling materials
        self.add_material(Material::new(
            "Ceiling Tile 600x600".to_string(),
            "CEIL_TILE_600".to_string(),
            Unit::Piece,
            12.50,
            Discipline::SuspendedCeiling,
            MaterialCategory::CeilingTile,
        ));

        self.add_material(Material::new(
            "T-Grid Main Profile".to_string(),
            "CEIL_GRID_MAIN".to_string(),
            Unit::Meter,
            8.50,
            Discipline::SuspendedCeiling,
            MaterialCategory::CeilingGrid,
        ));

        self.add_material(Material::new(
            "T-Grid Cross Profile".to_string(),
            "CEIL_GRID_CROSS".to_string(),
            Unit::Meter,
            6.50,
            Discipline::SuspendedCeiling,
            MaterialCategory::CeilingGrid,
        ));

        self.add_material(Material::new(
            "Ceiling Hanger".to_string(),
            "CEIL_HANGER".to_string(),
            Unit::Piece,
            1.20,
            Discipline::SuspendedCeiling,
            MaterialCategory::CeilingHanger,
        ));

        // Fixings
        self.add_material(Material::new(
            "Wall Anchor 8mm".to_string(),
            "ANCHOR_8MM".to_string(),
            Unit::Piece,
            0.25,
            Discipline::Architecture,
            MaterialCategory::Anchor,
        ));

        // Basic electrical materials
        self.add_material(Material::new(
            "NYM Cable 3x1.5mm".to_string(),
            "ELEC_NYM_315".to_string(),
            Unit::MeterCable,
            2.80,
            Discipline::Electrical,
            MaterialCategory::ElectricalCable,
        ));

        self.add_material(Material::new(
            "Standard Outlet".to_string(),
            "ELEC_OUTLET_STD".to_string(),
            Unit::Piece,
            8.50,
            Discipline::Electrical,
            MaterialCategory::ElectricalOutlet,
        ));
    }
}

impl Default for MaterialDatabase {
    fn default() -> Self {
        let mut db = Self::new();
        db.load_defaults();
        db
    }
}
