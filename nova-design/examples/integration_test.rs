use nova_biz::{CompanyProfile, Customer, PriceItem, Quote, QuoteLineItem, TaxRate};
use nova_bom::BomEngine;
use nova_core::{
    building::Building,
    ceilings::SuspendedCeiling,
    drywall::DrywallWall,
    electrical::{ElectricalDevice, ElectricalDeviceType},
    DesignElement, Phase,
};

/// Integration test demonstrating complete workflow
fn main() {
    println!("NovaDesign Integration Test");
    println!("===========================");

    // 1. Create a building project
    let mut building = Building::new("Casa Rossi".to_string(), "Via Roma 123, Milano".to_string());

    // 2. Add electrical devices
    let mut outlet1 =
        ElectricalDevice::new("Presa Cucina".to_string(), ElectricalDeviceType::Outlet);
    outlet1.set_phase(Phase::Nuovo);

    let mut outlet2 =
        ElectricalDevice::new("Presa Soggiorno".to_string(), ElectricalDeviceType::Outlet);
    outlet2.set_phase(Phase::Nuovo);

    let mut switch1 = ElectricalDevice::new(
        "Interruttore Cucina".to_string(),
        ElectricalDeviceType::Switch,
    );
    switch1.set_phase(Phase::Nuovo);

    println!("\n1. Building Project: {}", building.name);
    println!("   Address: {}", building.address);

    // 3. Add drywall walls
    let mut wall1 = DrywallWall::new("Parete Soggiorno".to_string(), 4.0, 2.8, 100.0);
    wall1.set_phase(Phase::Nuovo);

    let mut wall2 = DrywallWall::new("Parete Cucina".to_string(), 3.5, 2.8, 100.0);
    wall2.set_phase(Phase::Nuovo);

    println!("\n2. Drywall Walls:");
    println!(
        "   {} - {:.1}m x {:.1}m = {:.1}m²",
        wall1.name,
        wall1.length,
        wall1.height,
        wall1.area()
    );
    println!(
        "   {} - {:.1}m x {:.1}m = {:.1}m²",
        wall2.name,
        wall2.length,
        wall2.height,
        wall2.area()
    );

    // 4. Add suspended ceilings
    let mut ceiling1 = SuspendedCeiling::new("Controsoffitto Soggiorno".to_string(), 4.0, 3.5, 0.3);
    ceiling1.set_phase(Phase::Nuovo);

    println!("\n3. Suspended Ceilings:");
    println!(
        "   {} - {:.1}m x {:.1}m = {:.1}m²",
        ceiling1.name,
        ceiling1.length,
        ceiling1.width,
        ceiling1.area()
    );

    // 5. Generate BOM
    let mut bom_engine = BomEngine::new();

    // Add electrical devices to BOM
    let electrical_devices = vec![outlet1, outlet2, switch1];
    bom_engine.add_electrical_devices(&electrical_devices);

    // Add drywall materials to BOM
    let drywall_walls = vec![wall1, wall2];
    bom_engine.add_drywall_materials(&drywall_walls);

    // Add ceiling materials to BOM
    let ceilings = vec![ceiling1];
    bom_engine.add_suspended_ceilings(&ceilings);

    println!("\n4. Bill of Materials (BOM):");
    println!(
        "   {:<20} {:<30} {:<8} {:<10} {:<15}",
        "Code", "Description", "Unit", "Quantity", "Category"
    );
    println!("   {}", "-".repeat(85));

    for item in bom_engine.items() {
        println!(
            "   {:<20} {:<30} {:<8} {:<10.1} {:<15}",
            item.code, item.description, item.unit, item.quantity, item.category
        );
    }

    // 6. Create business entities
    let company = CompanyProfile::new(
        "Ditta Edile Bianchi".to_string(),
        "IT12345678901".to_string(),
        "Via Industria 45, Milano".to_string(),
        "info@bianchi-edile.it".to_string(),
        "+39 02 1234567".to_string(),
    );

    let customer = Customer::new(
        "Famiglia Rossi".to_string(),
        "RSSMRO80A01F205X".to_string(),
        "Via Roma 123, Milano".to_string(),
    );

    println!("\n5. Business Information:");
    println!("   Company: {}", company.name);
    println!("   Customer: {}", customer.name);

    // 7. Create quote with sample prices
    let mut quote = Quote::new("PREV001/2024".to_string(), customer, company);

    let iva_22 = TaxRate::iva_22();

    // Sample price items
    let price_outlet = PriceItem::new(
        "ELE_OUTLET".to_string(),
        "Presa elettrica".to_string(),
        "pz".to_string(),
        25.0,
        iva_22.clone(),
    );

    let price_switch = PriceItem::new(
        "ELE_SWITCH".to_string(),
        "Interruttore".to_string(),
        "pz".to_string(),
        15.0,
        iva_22.clone(),
    );

    let price_drywall_board = PriceItem::new(
        "DRY_BOARD".to_string(),
        "Lastra cartongesso".to_string(),
        "m²".to_string(),
        8.50,
        iva_22.clone(),
    );

    // Add line items to quote
    quote.add_line_item(QuoteLineItem::new(&price_outlet, 2.0)); // 2 outlets
    quote.add_line_item(QuoteLineItem::new(&price_switch, 1.0)); // 1 switch
    quote.add_line_item(QuoteLineItem::new(&price_drywall_board, 42.0)); // approximate m² of boards

    println!("\n6. Quote Summary:");
    println!("   Quote Number: {}", quote.number);
    println!("   Subtotal: €{:.2}", quote.subtotal());
    println!("   VAT: €{:.2}", quote.total_tax());
    println!("   Total: €{:.2}", quote.total());

    // 8. Export CSV (would normally save to file)
    if let Ok(csv_data) = quote.export_csv() {
        println!("\n7. CSV Export (first 200 chars):");
        println!("   {}", &csv_data[..csv_data.len().min(200)]);
        if csv_data.len() > 200 {
            println!("   ... (truncated)");
        }
    }

    println!("\n✅ NovaDesign integration test completed successfully!");
}
