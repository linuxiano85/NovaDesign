use gtk4::prelude::*;
use libadwaita as adw;

fn main() -> glib::ExitCode {
    let application = adw::Application::builder()
        .application_id("io.nova.Design")
        .build();

    application.connect_activate(build_ui);

    application.run()
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("NovaDesign")
        .default_width(800)
        .default_height(600)
        .build();

    let header_bar = adw::HeaderBar::new();

    let content = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let welcome_label = gtk4::Label::builder()
        .label("Welcome to NovaDesign!")
        .css_classes(vec!["title-1".to_string()])
        .build();

    content.append(&welcome_label);

    window.set_titlebar(Some(&header_bar));
    window.set_content(Some(&content));
    window.present();
}
