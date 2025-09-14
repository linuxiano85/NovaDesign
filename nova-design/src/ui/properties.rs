use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{Box, ComboBoxText, Label, Orientation};
use libadwaita as adw;

use crate::application::NovaApplication;

/// Build the properties panel
pub fn build_properties_panel(app: &NovaApplication) -> Box {
    let properties_box = Box::new(Orientation::Vertical, 12);
    properties_box.set_width_request(250);
    properties_box.set_margin_top(12);
    properties_box.set_margin_bottom(12);
    properties_box.set_margin_start(12);
    properties_box.set_margin_end(12);

    // Title
    let title = Label::new(Some(&app.translate("properties-title")));
    title.add_css_class("heading");
    title.set_halign(gtk4::Align::Start);
    properties_box.append(&title);

    // Phase selection
    let phase_label = Label::new(Some(&app.translate("properties-phase")));
    phase_label.set_halign(gtk4::Align::Start);
    phase_label.add_css_class("caption");
    properties_box.append(&phase_label);

    let phase_combo = ComboBoxText::new();
    phase_combo.append_text(&app.translate("phase-existing"));
    phase_combo.append_text(&app.translate("phase-demolition"));
    phase_combo.append_text(&app.translate("phase-new"));
    phase_combo.set_active(Some(2)); // Default to "Nuovo"
    properties_box.append(&phase_combo);

    // Placeholder for additional properties
    let placeholder_label = Label::new(Some(
        "Seleziona un elemento\nper visualizzare\nle sue proprietà",
    ));
    placeholder_label.add_css_class("dim-label");
    placeholder_label.set_justify(gtk4::Justification::Center);
    placeholder_label.set_halign(gtk4::Align::Center);
    placeholder_label.set_valign(gtk4::Align::Center);
    placeholder_label.set_vexpand(true);
    properties_box.append(&placeholder_label);

    properties_box
}
