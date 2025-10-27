use crate::{BomCalculator, BomItem, Result};
use nova_core::{ComponentType, Discipline, ElectricalComponent, MaterialDatabase, Project};

/// Electrical BOM calculator
#[derive(Default)]
pub struct ElectricalCalculator;

impl ElectricalCalculator {
    pub fn new() -> Self {
        Self
    }

    /// Count electrical components by type
    fn count_electrical_components(
        &self,
        project: &Project,
    ) -> std::collections::HashMap<ElectricalComponent, usize> {
        let mut counts = std::collections::HashMap::new();

        for building in &project.buildings {
            for floor in &building.floors {
                for component in &floor.components {
                    if component.discipline == Discipline::Electrical {
                        if let ComponentType::Electrical(elec_type) = &component.component_type {
                            *counts.entry(elec_type.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        counts
    }

    /// Calculate cable length needed
    fn calculate_cable_length(&self, project: &Project) -> f64 {
        let mut total_length = 0.0;

        for building in &project.buildings {
            for floor in &building.floors {
                let electrical_components: Vec<_> = floor
                    .components
                    .iter()
                    .filter(|c| c.discipline == Discipline::Electrical)
                    .collect();

                if electrical_components.is_empty() {
                    continue;
                }

                // Basic cable length estimation
                // This is a simplified calculation - in reality, you'd need proper routing
                let _component_count = electrical_components.len();

                // Estimate average cable runs
                // - From panel to outlets: average 10m per outlet
                // - Between outlets: average 5m per connection
                // - To lighting: average 8m per light

                for component in &electrical_components {
                    if let ComponentType::Electrical(elec_type) = &component.component_type {
                        let estimated_cable = match elec_type {
                            ElectricalComponent::Outlet => 10.0,  // 10m average run
                            ElectricalComponent::Switch => 8.0,   // 8m average run
                            ElectricalComponent::Light => 12.0,   // 12m average run (ceiling)
                            ElectricalComponent::Panel => 0.0,    // Panel doesn't need cable TO it
                            ElectricalComponent::Junction => 5.0, // 5m average
                            ElectricalComponent::Conduit => 1.0,  // 1m per conduit component
                        };
                        total_length += estimated_cable;
                    }
                }
            }
        }

        // Add 20% for waste and routing inefficiencies
        total_length * 1.2
    }
}

impl BomCalculator for ElectricalCalculator {
    fn calculate(&self, project: &Project, material_db: &MaterialDatabase) -> Result<Vec<BomItem>> {
        let mut items = Vec::new();

        let component_counts = self.count_electrical_components(project);
        let cable_length = self.calculate_cable_length(project);

        // Only calculate if there are electrical components
        if !component_counts.is_empty() {
            // Cable calculation
            if cable_length > 0.0 {
                if let Some(cable_material) = material_db.get_material_by_code("ELEC_NYM_315") {
                    items.push(BomItem::new(
                        cable_material,
                        cable_length,
                        Some("Estimated cable runs with 20% waste factor".to_string()),
                    ));
                }
            }

            // Outlets
            if let Some(outlet_count) = component_counts.get(&ElectricalComponent::Outlet) {
                if let Some(outlet_material) = material_db.get_material_by_code("ELEC_OUTLET_STD") {
                    items.push(BomItem::new(
                        outlet_material,
                        *outlet_count as f64,
                        Some("Standard electrical outlets".to_string()),
                    ));
                }
            }

            // For other electrical components, we would add similar calculations
            // This is a basic implementation showing the pattern
        }

        Ok(items)
    }

    fn discipline(&self) -> Discipline {
        Discipline::Electrical
    }
}
