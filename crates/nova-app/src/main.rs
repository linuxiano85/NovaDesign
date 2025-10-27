use adw::prelude::*;
use gtk4 as gtk;
use once_cell::sync::Lazy;
use tracing_subscriber::EnvFilter;

use nova_core::{Discipline, Phase, Project};

static APP_ID: &str = "io.nova.Design";

static INIT_TRACING: Lazy<()> = Lazy::new(|| {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
});

mod application;
mod ui;

use application::NovaApplication;

fn main() -> glib::ExitCode {
    Lazy::force(&INIT_TRACING);

    tracing::info!("Starting Nova Design application");

    // Initialize GTK and Adwaita
    gtk::init().expect("Failed to initialize GTK");
    adw::init().expect("Failed to initialize Libadwaita");

    // Create and run the application
    let app = NovaApplication::new();
    app.run()
}
