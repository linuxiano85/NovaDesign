use crate::{BomCalculator, BomItem, Result};
use nova_core::{ComponentType, Discipline, MaterialDatabase, Project, SuspendedCeilingComponent};

/// Suspended ceiling BOM calculator
#[derive(Default)]
pub struct SuspendedCeilingCalculator;

impl SuspendedCeilingCalculator {
    pub fn new() -> Self {
        Self
    }

    /// Calculate ceiling area
    fn calculate_ceiling_area(&self, project: &Project) -> f64 {
        let mut total_area = 0.0;

        for building in &project.buildings {
            for floor in &building.floors {
                // Calculate area of suspended ceiling components
                for component in &floor.components {
                    if component.discipline == Discipline::SuspendedCeiling {
                        if let ComponentType::SuspendedCeiling(SuspendedCeilingComponent::Tile) =
                            &component.component_type
                        {
                            // Standard tile size 600x600mm = 0.36 m²
                            total_area += 0.36;
                        }
                    }
                }

                // For this basic implementation, also consider floor area if there are ceiling components
                let has_ceiling_components = floor
                    .components
                    .iter()
                    .any(|c| c.discipline == Discipline::SuspendedCeiling);

                if has_ceiling_components {
                    // Calculate approximate floor area from walls (very basic)
                    let mut min_x = f64::MAX;
                    let mut max_x = f64::MIN;
                    let mut min_y = f64::MAX;
                    let mut max_y = f64::MIN;

                    for wall in &floor.walls {
                        min_x = min_x.min(wall.start_point.x.min(wall.end_point.x));
                        max_x = max_x.max(wall.start_point.x.max(wall.end_point.x));
                        min_y = min_y.min(wall.start_point.y.min(wall.end_point.y));
                        max_y = max_y.max(wall.start_point.y.max(wall.end_point.y));
                    }

                    if min_x != f64::MAX && max_x != f64::MIN {
                        let area = (max_x - min_x) * (max_y - min_y);
                        total_area += area;
                    }
                }
            }
        }

        total_area
    }

    /// Calculate number of ceiling tiles needed
    fn calculate_tile_count(&self, ceiling_area: f64) -> f64 {
        // Standard tile size 600x600mm = 0.36 m²
        // Add 5% waste factor
        let tile_area = 0.36;
        let tiles_needed = ceiling_area / tile_area;
        tiles_needed * 1.05 // 5% waste
    }

    /// Calculate T-grid main profile length
    fn calculate_main_grid_length(&self, _project: &Project, ceiling_area: f64) -> f64 {
        if ceiling_area == 0.0 {
            return 0.0;
        }

        // Estimate based on typical grid layout
        // Main profiles typically run 1200mm apart
        // This is a simplified calculation
        let typical_room_width = (ceiling_area * 0.75).sqrt(); // Assume roughly square rooms
        let main_profile_count = (typical_room_width / 1.2).ceil();
        main_profile_count * typical_room_width
    }

    /// Calculate T-grid cross profile length
    fn calculate_cross_grid_length(&self, _project: &Project, ceiling_area: f64) -> f64 {
        if ceiling_area == 0.0 {
            return 0.0;
        }

        // Cross profiles run perpendicular to main profiles at 600mm spacing
        let typical_room_length = (ceiling_area * 1.33).sqrt(); // Assume roughly rectangular
        let cross_profile_count = (typical_room_length / 0.6).ceil();
        cross_profile_count * (ceiling_area / typical_room_length)
    }

    /// Calculate hanger count
    fn calculate_hanger_count(&self, ceiling_area: f64) -> f64 {
        if ceiling_area == 0.0 {
            return 0.0;
        }

        // Standard hanger spacing: 1200mm x 1200mm grid
        // Add extra for perimeter support
        let hangers_per_sqm = 1.0 / (1.2 * 1.2); // hangers per m²
        ceiling_area * hangers_per_sqm * 1.1 // 10% extra for perimeter
    }

    /// Calculate fixings for hangers
    fn calculate_hanger_fixings(&self, hanger_count: f64) -> f64 {
        // Each hanger typically needs 1 fixing to structure
        hanger_count
    }
}

impl BomCalculator for SuspendedCeilingCalculator {
    fn calculate(&self, project: &Project, material_db: &MaterialDatabase) -> Result<Vec<BomItem>> {
        let mut items = Vec::new();

        let ceiling_area = self.calculate_ceiling_area(project);

        // Only calculate if there's actual ceiling work
        if ceiling_area > 0.0 {
            let tile_count = self.calculate_tile_count(ceiling_area);
            let main_grid_length = self.calculate_main_grid_length(project, ceiling_area);
            let cross_grid_length = self.calculate_cross_grid_length(project, ceiling_area);
            let hanger_count = self.calculate_hanger_count(ceiling_area);
            let fixing_count = self.calculate_hanger_fixings(hanger_count);

            // Ceiling tiles
            if let Some(tile_material) = material_db.get_material_by_code("CEIL_TILE_600") {
                items.push(BomItem::new(
                    tile_material,
                    tile_count,
                    Some("600x600mm tiles with 5% waste factor".to_string()),
                ));
            }

            // Main grid profiles
            if let Some(main_grid_material) = material_db.get_material_by_code("CEIL_GRID_MAIN") {
                items.push(BomItem::new(
                    main_grid_material,
                    main_grid_length,
                    Some("Main T-grid profiles, 1200mm spacing".to_string()),
                ));
            }

            // Cross grid profiles
            if let Some(cross_grid_material) = material_db.get_material_by_code("CEIL_GRID_CROSS") {
                items.push(BomItem::new(
                    cross_grid_material,
                    cross_grid_length,
                    Some("Cross T-grid profiles, 600mm spacing".to_string()),
                ));
            }

            // Hangers
            if let Some(hanger_material) = material_db.get_material_by_code("CEIL_HANGER") {
                items.push(BomItem::new(
                    hanger_material,
                    hanger_count,
                    Some("Ceiling hangers, 1200x1200mm grid".to_string()),
                ));
            }

            // Fixings for hangers
            if let Some(anchor_material) = material_db.get_material_by_code("ANCHOR_8MM") {
                items.push(BomItem::new(
                    anchor_material,
                    fixing_count,
                    Some("Hanger fixings to structure".to_string()),
                ));
            }
        }

        Ok(items)
    }

    fn discipline(&self) -> Discipline {
        Discipline::SuspendedCeiling
    }
}
