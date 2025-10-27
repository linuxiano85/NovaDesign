use nova_core::{Discipline, EntityId, Material, MaterialDatabase, Project, Unit};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod drywall;
pub mod electrical;
pub mod suspended_ceiling;

pub use drywall::*;
pub use electrical::*;
pub use suspended_ceiling::*;

/// BOM calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomCalculation {
    pub project_id: EntityId,
    pub items: Vec<BomItem>,
    pub total_cost: f64,
    pub calculation_date: String,
}

/// Individual BOM item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomItem {
    pub material_code: String,
    pub material_name: String,
    pub quantity: f64,
    pub unit: Unit,
    pub unit_cost: f64,
    pub total_cost: f64,
    pub discipline: Discipline,
    pub category: String,
    pub notes: Option<String>,
}

impl BomItem {
    pub fn new(material: &Material, quantity: f64, notes: Option<String>) -> Self {
        let total_cost = quantity * material.cost_per_unit;
        Self {
            material_code: material.code.clone(),
            material_name: material.name.clone(),
            quantity,
            unit: material.unit,
            unit_cost: material.cost_per_unit,
            total_cost,
            discipline: material.discipline,
            category: format!("{:?}", material.category),
            notes,
        }
    }
}

/// Main BOM calculation engine
pub struct BomEngine {
    material_db: MaterialDatabase,
    calculators: Vec<Box<dyn BomCalculator>>,
}

impl BomEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            material_db: MaterialDatabase::default(),
            calculators: Vec::new(),
        };

        // Register default calculators
        engine.calculators.push(Box::new(DrywallCalculator::new()));
        engine
            .calculators
            .push(Box::new(SuspendedCeilingCalculator::new()));
        engine
            .calculators
            .push(Box::new(ElectricalCalculator::new()));

        engine
    }

    pub fn calculate_bom(&self, project: &Project) -> Result<BomCalculation> {
        let mut all_items = Vec::new();

        // Run all calculators
        for calculator in &self.calculators {
            let mut items = calculator.calculate(project, &self.material_db)?;
            all_items.append(&mut items);
        }

        // Consolidate items with same material code
        let mut consolidated: HashMap<String, BomItem> = HashMap::new();
        for item in all_items {
            let key = item.material_code.clone();
            if let Some(existing) = consolidated.get_mut(&key) {
                existing.quantity += item.quantity;
                existing.total_cost += item.total_cost;
            } else {
                consolidated.insert(key, item);
            }
        }

        let items: Vec<BomItem> = consolidated.into_values().collect();
        let total_cost = items.iter().map(|item| item.total_cost).sum();

        Ok(BomCalculation {
            project_id: project.id,
            items,
            total_cost,
            calculation_date: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub fn export_csv(&self, bom: &BomCalculation) -> Result<String> {
        let mut wtr = csv::Writer::from_writer(Vec::new());

        // Write header
        wtr.write_record([
            "Material Code",
            "Material Name",
            "Quantity",
            "Unit",
            "Unit Cost",
            "Total Cost",
            "Discipline",
            "Category",
            "Notes",
        ])?;

        // Write items
        for item in &bom.items {
            wtr.write_record([
                &item.material_code,
                &item.material_name,
                &item.quantity.to_string(),
                &format!("{:?}", item.unit),
                &item.unit_cost.to_string(),
                &item.total_cost.to_string(),
                &format!("{:?}", item.discipline),
                &item.category,
                item.notes.as_deref().unwrap_or(""),
            ])?;
        }

        let data = wtr.into_inner().unwrap();
        Ok(String::from_utf8(data)?)
    }
}

impl Default for BomEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for discipline-specific BOM calculators
pub trait BomCalculator: Send + Sync {
    fn calculate(&self, project: &Project, material_db: &MaterialDatabase) -> Result<Vec<BomItem>>;
    fn discipline(&self) -> Discipline;
}

/// BOM calculation errors
#[derive(Debug, thiserror::Error)]
pub enum BomError {
    #[error("Material not found: {code}")]
    MaterialNotFound { code: String },
    #[error("Calculation error: {message}")]
    CalculationError { message: String },
    #[error("CSV export error: {0}")]
    CsvError(#[from] csv::Error),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, BomError>;
