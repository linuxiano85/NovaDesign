use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ListBox, Orientation, ScrolledWindow};
use libadwaita as adw;

use crate::application::NovaApplication;

/// Build the sidebar with discipline categories
pub fn build_sidebar(app: &NovaApplication) -> Box {
    let sidebar_box = Box::new(Orientation::Vertical, 6);
    sidebar_box.set_width_request(250);
    sidebar_box.add_css_class("sidebar");

    // Title
    let title = Label::new(Some("Discipline"));
    title.add_css_class("heading");
    title.set_margin_top(12);
    title.set_margin_bottom(6);
    sidebar_box.append(&title);

    // Disciplines list
    let disciplines_list = ListBox::new();
    disciplines_list.add_css_class("navigation-sidebar");

    // Add discipline buttons
    let disciplines = [
        ("discipline-architecture", "🏗️"),
        ("discipline-electrical", "⚡"),
        ("discipline-plumbing", "🚿"),
        ("discipline-masonry", "🧱"),
        ("discipline-drywall", "🔳"),
        ("discipline-ceilings", "📐"),
        ("discipline-painting", "🎨"),
        ("discipline-structural", "🏢"),
    ];

    for (key, icon) in disciplines {
        let row = create_discipline_row(app, key, icon);
        disciplines_list.append(&row);
    }

    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&disciplines_list));
    scrolled.set_vexpand(true);

    sidebar_box.append(&scrolled);

    sidebar_box
}

fn create_discipline_row(app: &NovaApplication, translate_key: &str, icon: &str) -> Box {
    let row_box = Box::new(Orientation::Horizontal, 12);
    row_box.set_margin_top(6);
    row_box.set_margin_bottom(6);
    row_box.set_margin_start(12);
    row_box.set_margin_end(12);

    // Icon
    let icon_label = Label::new(Some(icon));
    icon_label.set_width_request(24);
    row_box.append(&icon_label);

    // Text
    let text_label = Label::new(Some(&app.translate(translate_key)));
    text_label.set_hexpand(true);
    text_label.set_halign(gtk4::Align::Start);
    row_box.append(&text_label);

    // Make the whole row clickable
    let button = Button::new();
    button.set_child(Some(&row_box));
    button.add_css_class("flat");
    button.set_hexpand(true);

    // Connect click handler (placeholder for now)
    let translate_key_owned = translate_key.to_string();
    button.connect_clicked(move |_| {
        println!("Clicked discipline: {}", translate_key_owned);
    });

    let container = Box::new(Orientation::Vertical, 0);
    container.append(&button);
    container
}
