use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};
use libadwaita as adw;

use crate::application::NovaApplication;

/// Build the 2D view placeholder
pub fn build_2d_view(app: &NovaApplication) -> Box {
    let view_box = Box::new(Orientation::Vertical, 24);
    view_box.set_hexpand(true);
    view_box.set_vexpand(true);
    view_box.set_halign(gtk4::Align::Center);
    view_box.set_valign(gtk4::Align::Center);

    // Icon placeholder
    let icon = Label::new(Some("📐"));
    icon.add_css_class("large-title");
    view_box.append(&icon);

    // Title
    let title = Label::new(Some(&app.translate("view-2d")));
    title.add_css_class("title-1");
    view_box.append(&title);

    // Description
    let description = Label::new(Some(
        "Vista planimetrica 2D\n(Placeholder - Rendering con wgpu in arrivo)",
    ));
    description.add_css_class("body");
    description.set_justify(gtk4::Justification::Center);
    view_box.append(&description);

    view_box
}

/// Build the 3D view placeholder
pub fn build_3d_view(app: &NovaApplication) -> Box {
    let view_box = Box::new(Orientation::Vertical, 24);
    view_box.set_hexpand(true);
    view_box.set_vexpand(true);
    view_box.set_halign(gtk4::Align::Center);
    view_box.set_valign(gtk4::Align::Center);

    // Icon placeholder
    let icon = Label::new(Some("🏢"));
    icon.add_css_class("large-title");
    view_box.append(&icon);

    // Title
    let title = Label::new(Some(&app.translate("view-3d")));
    title.add_css_class("title-1");
    view_box.append(&title);

    // Description
    let description = Label::new(Some(
        "Vista tridimensionale 3D\n(Placeholder - Rendering con wgpu in arrivo)",
    ));
    description.add_css_class("body");
    description.set_justify(gtk4::Justification::Center);
    view_box.append(&description);

    view_box
}
