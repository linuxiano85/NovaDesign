use nova_biz::{CompanyProfile, Customer, PriceList, Quote};
use nova_bom::BomEngine;
use nova_core::{
    building::Building, ceilings::SuspendedCeiling, drywall::DrywallWall,
    electrical::ElectricalDevice, plumbing::PlumbingDevice,
};
use nova_i18n::{I18nManager, Language};

/// Main application state
pub struct NovaApplication {
    pub building: Option<Building>,
    pub bom_engine: BomEngine,
    pub i18n: I18nManager,
    pub company_profile: Option<CompanyProfile>,
    pub customers: Vec<Customer>,
    pub price_lists: Vec<PriceList>,
    pub quotes: Vec<Quote>,
}

impl NovaApplication {
    pub fn new() -> Self {
        Self {
            building: None,
            bom_engine: BomEngine::new(),
            i18n: I18nManager::new(),
            company_profile: None,
            customers: Vec::new(),
            price_lists: Vec::new(),
            quotes: Vec::new(),
        }
    }

    pub fn create_new_building(&mut self, name: String, address: String) {
        self.building = Some(Building::new(name, address));
    }

    pub fn get_building(&self) -> Option<&Building> {
        self.building.as_ref()
    }

    pub fn get_building_mut(&mut self) -> Option<&mut Building> {
        self.building.as_mut()
    }

    pub fn generate_bom(&mut self) {
        if let Some(building) = &self.building {
            self.bom_engine.clear();

            // Collect electrical devices
            let electrical_devices: Vec<ElectricalDevice> = Vec::new(); // TODO: collect from building
            self.bom_engine.add_electrical_devices(&electrical_devices);

            // Collect drywall walls
            let drywall_walls: Vec<DrywallWall> = Vec::new(); // TODO: collect from building
            self.bom_engine.add_drywall_materials(&drywall_walls);

            // Collect suspended ceilings
            let ceilings: Vec<SuspendedCeiling> = Vec::new(); // TODO: collect from building
            self.bom_engine.add_suspended_ceilings(&ceilings);
        }
    }

    pub fn set_language(&mut self, language: Language) {
        self.i18n.set_language(language);
    }

    pub fn translate(&self, key: &str) -> String {
        self.i18n.get(key)
    }
}

impl Default for NovaApplication {
    fn default() -> Self {
        Self::new()
    }
}
