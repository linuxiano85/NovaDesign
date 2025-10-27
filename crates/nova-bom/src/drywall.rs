use crate::{BomCalculator, BomItem, Result};
use nova_core::{ComponentType, Discipline, DrywallComponent, MaterialDatabase, Project};

/// Drywall BOM calculator
#[derive(Default)]
pub struct DrywallCalculator;

impl DrywallCalculator {
    pub fn new() -> Self {
        Self
    }

    /// Calculate drywall area for a floor
    fn calculate_drywall_area(&self, project: &Project) -> f64 {
        let mut total_area = 0.0;

        for building in &project.buildings {
            for floor in &building.floors {
                // Calculate area of drywall components
                for component in &floor.components {
                    if component.discipline == Discipline::Drywall {
                        if let ComponentType::Drywall(DrywallComponent::Panel) =
                            &component.component_type
                        {
                            // For this basic implementation, assume each panel component represents 1m²
                            // In a real implementation, this would use actual panel dimensions
                            total_area += 1.0;
                        }
                    }
                }

                // Also calculate from walls that are drywall
                for wall in &floor.walls {
                    if wall.disciplines.contains(&Discipline::Drywall) {
                        total_area += wall.area();
                    }
                }
            }
        }

        total_area
    }

    /// Calculate stud length needed
    fn calculate_stud_length(&self, project: &Project) -> f64 {
        let mut total_length = 0.0;

        for building in &project.buildings {
            for floor in &building.floors {
                for wall in &floor.walls {
                    if wall.disciplines.contains(&Discipline::Drywall) {
                        let wall_length = wall.length();
                        // Standard stud spacing: 400mm or 600mm centers
                        // For this calculation, use 400mm spacing
                        let stud_count = (wall_length / 0.4).ceil();
                        total_length += stud_count * wall.height;
                    }
                }
            }
        }

        total_length
    }

    /// Calculate track length (perimeter tracks)
    fn calculate_track_length(&self, project: &Project) -> f64 {
        let mut total_length = 0.0;

        for building in &project.buildings {
            for floor in &building.floors {
                for wall in &floor.walls {
                    if wall.disciplines.contains(&Discipline::Drywall) {
                        // Top and bottom tracks
                        total_length += wall.length() * 2.0;
                    }
                }
            }
        }

        total_length
    }

    /// Calculate number of screws needed
    fn calculate_screws(&self, drywall_area: f64) -> f64 {
        // Standard: approximately 25 screws per m² of drywall
        drywall_area * 25.0
    }

    /// Calculate fixings (tasselli) based on substrate
    fn calculate_fixings(&self, project: &Project) -> f64 {
        let mut fixing_count = 0.0;

        for building in &project.buildings {
            for floor in &building.floors {
                for wall in &floor.walls {
                    if wall.disciplines.contains(&Discipline::Drywall) {
                        let wall_length = wall.length();

                        // Fixing spacing depends on substrate
                        // For concrete/masonry: every 600mm
                        // For this basic implementation, assume concrete substrate
                        let spacing = 0.6; // meters
                        let fixings_per_wall = (wall_length / spacing).ceil() * 2.0; // top and bottom tracks
                        fixing_count += fixings_per_wall;
                    }
                }
            }
        }

        fixing_count
    }
}

impl BomCalculator for DrywallCalculator {
    fn calculate(&self, project: &Project, material_db: &MaterialDatabase) -> Result<Vec<BomItem>> {
        let mut items = Vec::new();

        let drywall_area = self.calculate_drywall_area(project);
        let stud_length = self.calculate_stud_length(project);
        let track_length = self.calculate_track_length(project);
        let screw_count = self.calculate_screws(drywall_area);
        let fixing_count = self.calculate_fixings(project);

        // Only calculate if there's actual drywall work
        if drywall_area > 0.0 {
            // Drywall panels
            if let Some(panel_material) = material_db.get_material_by_code("DW_PANEL_125") {
                items.push(BomItem::new(
                    panel_material,
                    drywall_area,
                    Some("Calculated from wall areas and drywall components".to_string()),
                ));
            }

            // Metal studs
            if let Some(stud_material) = material_db.get_material_by_code("DW_STUD_75") {
                items.push(BomItem::new(
                    stud_material,
                    stud_length,
                    Some("400mm spacing, standard height".to_string()),
                ));
            }

            // Metal tracks
            if let Some(track_material) = material_db.get_material_by_code("DW_TRACK_75") {
                items.push(BomItem::new(
                    track_material,
                    track_length,
                    Some("Top and bottom perimeter tracks".to_string()),
                ));
            }

            // Screws
            if let Some(screw_material) = material_db.get_material_by_code("DW_SCREW_25") {
                items.push(BomItem::new(
                    screw_material,
                    screw_count,
                    Some("25 screws per m² of drywall".to_string()),
                ));
            }

            // Fixings
            if let Some(anchor_material) = material_db.get_material_by_code("ANCHOR_8MM") {
                items.push(BomItem::new(
                    anchor_material,
                    fixing_count,
                    Some("Track fixings to structure, 600mm spacing".to_string()),
                ));
            }
        }

        Ok(items)
    }

    fn discipline(&self) -> Discipline {
        Discipline::Drywall
    }
}
