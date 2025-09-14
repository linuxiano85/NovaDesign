use adw::prelude::*;
use glib::Object;
use gtk4 as gtk;
use std::cell::RefCell;
use std::rc::Rc;

use crate::ui::window::NovaWindow;
use nova_core::Project;
use nova_i18n::{Language, Localizer};

const APP_ID: &str = "io.nova.Design";

glib::wrapper! {
    pub struct NovaApplication(ObjectSubclass<imp::NovaApplication>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl NovaApplication {
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", APP_ID)
            .property("flags", gio::ApplicationFlags::HANDLES_OPEN)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();

        let new_project_action = gio::ActionEntry::builder("new-project")
            .activate(move |app: &Self, _, _| app.new_project())
            .build();

        let open_project_action = gio::ActionEntry::builder("open-project")
            .activate(move |app: &Self, _, _| app.open_project())
            .build();

        let save_project_action = gio::ActionEntry::builder("save-project")
            .activate(move |app: &Self, _, _| app.save_project())
            .build();

        self.add_action_entries([
            quit_action,
            about_action,
            new_project_action,
            open_project_action,
            save_project_action,
        ]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("Nova Design")
            .application_icon(APP_ID)
            .developer_name("Nova Design Contributors")
            .version("0.1.0")
            .developers(vec!["Nova Design Contributors"])
            .copyright("© 2024 Nova Design Contributors")
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/linuxiano85/NovaDesign")
            .issue_url("https://github.com/linuxiano85/NovaDesign/issues")
            .comments("Linux-only CAD/BIM light for trades, built in Rust.")
            .build();

        about.present();
    }

    fn new_project(&self) {
        if let Some(window) = self.active_window() {
            if let Some(nova_window) = window.downcast_ref::<NovaWindow>() {
                nova_window.new_project();
            }
        }
    }

    fn open_project(&self) {
        if let Some(window) = self.active_window() {
            if let Some(nova_window) = window.downcast_ref::<NovaWindow>() {
                nova_window.open_project();
            }
        }
    }

    fn save_project(&self) {
        if let Some(window) = self.active_window() {
            if let Some(nova_window) = window.downcast_ref::<NovaWindow>() {
                nova_window.save_project();
            }
        }
    }
}

impl Default for NovaApplication {
    fn default() -> Self {
        Self::new()
    }
}

mod imp {
    use super::*;
    use std::cell::OnceCell;

    #[derive(Default)]
    pub struct NovaApplication {
        pub localizer: OnceCell<Rc<RefCell<Localizer>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NovaApplication {
        const NAME: &'static str = "NovaApplication";
        type Type = super::NovaApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for NovaApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.new-project", &["<primary>n"]);
            obj.set_accels_for_action("app.open-project", &["<primary>o"]);
            obj.set_accels_for_action("app.save-project", &["<primary>s"]);

            // Initialize localizer
            let localizer = Rc::new(RefCell::new(Localizer::new()));
            self.localizer.set(localizer).unwrap();
        }
    }

    impl ApplicationImpl for NovaApplication {
        fn activate(&self) {
            let application = self.obj();
            let window = NovaWindow::new(&application);
            window.present();
        }

        fn open(&self, files: &[gio::File], _hint: &str) {
            let application = self.obj();
            let window = NovaWindow::new(&application);

            // TODO: Open the specified files
            for file in files {
                tracing::info!("Opening file: {:?}", file.path());
            }

            window.present();
        }
    }

    impl GtkApplicationImpl for NovaApplication {}
    impl AdwApplicationImpl for NovaApplication {}
}
