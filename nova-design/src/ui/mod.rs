use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{Box, Button, ComboBoxText, Label, Notebook, Orientation, Paned, ScrolledWindow};
use libadwaita as adw;

use crate::application::NovaApplication;

pub mod business;
pub mod properties;
pub mod sidebar;
pub mod views;

use business::build_business_panel;
use properties::build_properties_panel;
use sidebar::build_sidebar;
use views::{build_2d_view, build_3d_view};

/// Build the main UI layout
pub fn build_main_ui(app: &NovaApplication) -> Box {
    let main_box = Box::new(Orientation::Vertical, 0);

    // Header bar
    let header_bar = build_header_bar(app);
    main_box.append(&header_bar);

    // Main content area with three panels
    let main_paned = Paned::new(Orientation::Horizontal);

    // Left panel: Sidebar with disciplines
    let sidebar = build_sidebar(app);
    main_paned.set_start_child(Some(&sidebar));

    // Center panel: Views (2D/3D) and main workspace
    let center_panel = build_center_panel(app);
    main_paned.set_end_child(Some(&center_panel));

    main_box.append(&main_paned);

    main_box
}

fn build_header_bar(app: &NovaApplication) -> adw::HeaderBar {
    let header_bar = adw::HeaderBar::new();

    // Title
    header_bar.set_title_widget(Some(&Label::new(Some(&app.translate("app-name")))));

    // Left side buttons
    let new_button = Button::with_label(&app.translate("toolbar-new"));
    let open_button = Button::with_label(&app.translate("toolbar-open"));
    let save_button = Button::with_label(&app.translate("toolbar-save"));

    header_bar.pack_start(&new_button);
    header_bar.pack_start(&open_button);
    header_bar.pack_start(&save_button);

    // Right side buttons
    let business_button = Button::with_label(&app.translate("business-title"));
    business_button.add_css_class("suggested-action");
    header_bar.pack_end(&business_button);

    header_bar
}

fn build_center_panel(app: &NovaApplication) -> Paned {
    let center_paned = Paned::new(Orientation::Horizontal);

    // Main view area with tabs
    let main_notebook = build_main_notebook(app);
    center_paned.set_start_child(Some(&main_notebook));

    // Right panel: Properties
    let properties_panel = build_properties_panel(app);
    center_paned.set_end_child(Some(&properties_panel));

    // Set reasonable proportions
    center_paned.set_position(800); // Give more space to main view

    center_paned
}

fn build_main_notebook(app: &NovaApplication) -> Notebook {
    let notebook = Notebook::new();
    notebook.set_hexpand(true);
    notebook.set_vexpand(true);

    // 2D View tab
    let view_2d = build_2d_view(app);
    let label_2d = Label::new(Some(&app.translate("view-2d")));
    notebook.append_page(&view_2d, Some(&label_2d));

    // 3D View tab
    let view_3d = build_3d_view(app);
    let label_3d = Label::new(Some(&app.translate("view-3d")));
    notebook.append_page(&view_3d, Some(&label_3d));

    // Business tab (hidden by default, shown when needed)
    let business_view = build_business_panel(app);
    let label_business = Label::new(Some(&app.translate("business-title")));
    notebook.append_page(&business_view, Some(&label_business));

    notebook
}
