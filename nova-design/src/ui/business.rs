use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ListBox, Orientation, ScrolledWindow};
use libadwaita as adw;

use crate::application::NovaApplication;

/// Build the business suite panel
pub fn build_business_panel(app: &NovaApplication) -> Box {
    let business_box = Box::new(Orientation::Vertical, 24);
    business_box.set_margin_top(24);
    business_box.set_margin_bottom(24);
    business_box.set_margin_start(24);
    business_box.set_margin_end(24);

    // Title
    let title = Label::new(Some(&app.translate("business-title")));
    title.add_css_class("title-1");
    title.set_halign(gtk4::Align::Center);
    business_box.append(&title);

    // Business functions grid
    let functions_box = Box::new(Orientation::Vertical, 12);

    // Quotes section
    let quotes_section = create_business_section(
        app,
        "💰",
        &app.translate("business-quotes"),
        "Crea e gestisci preventivi per i tuoi clienti",
    );
    functions_box.append(&quotes_section);

    // Invoices section
    let invoices_section = create_business_section(
        app,
        "📄",
        &app.translate("business-invoices"),
        "Genera fatture e gestisci la contabilità",
    );
    functions_box.append(&invoices_section);

    // DDT section
    let ddt_section = create_business_section(
        app,
        "📦",
        &app.translate("business-ddt"),
        "Documenti di trasporto per le consegne",
    );
    functions_box.append(&ddt_section);

    // Customers section
    let customers_section = create_business_section(
        app,
        "👥",
        &app.translate("business-customers"),
        "Gestisci anagrafica clienti e fornitori",
    );
    functions_box.append(&customers_section);

    // Price lists section
    let pricelists_section = create_business_section(
        app,
        "📋",
        &app.translate("business-pricelists"),
        "Listini prezzi e categorie merceologiche",
    );
    functions_box.append(&pricelists_section);

    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&functions_box));
    scrolled.set_vexpand(true);

    business_box.append(&scrolled);

    business_box
}

fn create_business_section(
    app: &NovaApplication,
    icon: &str,
    title: &str,
    description: &str,
) -> adw::ActionRow {
    let row = adw::ActionRow::new();
    row.set_title(title);
    row.set_subtitle(description);

    // Icon
    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("title-2");
    row.add_prefix(&icon_label);

    // Action button
    let action_button = Button::with_label("Apri");
    action_button.add_css_class("suggested-action");
    action_button.set_valign(gtk4::Align::Center);
    row.add_suffix(&action_button);

    // Connect click handler (placeholder for now)
    let title_clone = title.to_string();
    action_button.connect_clicked(move |_| {
        println!("Opening: {}", title_clone);
    });

    row
}
