use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use libadwaita as adw;

mod application;
mod ui;

use crate::application::NovaApplication;

const APP_ID: &str = "io.nova.Design";

fn main() -> glib::ExitCode {
    // Initialize i18n
    let _i18n = nova_i18n::I18nManager::new();

    // Create application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    // Initialize Adwaita
    adw::init().expect("Failed to initialize Adwaita");

    // Create main application
    let nova_app = NovaApplication::new();

    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("NovaDesign")
        .default_width(1200)
        .default_height(800)
        .build();

    // Build UI
    let main_view = ui::build_main_ui(&nova_app);
    window.set_child(Some(&main_view));

    // Show window
    window.present();
}
