use adw::prelude::*;
use glib::Object;
use gtk4 as gtk;
use std::cell::RefCell;
use std::rc::Rc;

use crate::application::NovaApplication;
use crate::ui::workspace::WorkspaceView;
use nova_bom::BomEngine;
use nova_core::Project;

glib::wrapper! {
    pub struct NovaWindow(ObjectSubclass<imp::NovaWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl NovaWindow {
    pub fn new(app: &NovaApplication) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn new_project(&self) {
        let imp = self.imp();
        let project = Project::new(
            "New Project".to_string(),
            "A new Nova Design project".to_string(),
        );
        imp.project.replace(Some(project));
        imp.workspace.set_project(imp.project.borrow().as_ref());
        self.update_title();
    }

    pub fn open_project(&self) {
        // TODO: Implement file dialog to open project
        tracing::info!("Open project requested");

        // For now, create a sample project
        self.new_project();
    }

    pub fn save_project(&self) {
        // TODO: Implement file dialog to save project
        tracing::info!("Save project requested");

        if let Some(project) = &*self.imp().project.borrow() {
            tracing::info!("Saving project: {}", project.name);
        }
    }

    fn update_title(&self) {
        let title = if let Some(project) = &*self.imp().project.borrow() {
            format!("Nova Design - {}", project.name)
        } else {
            "Nova Design".to_string()
        };
        self.set_title(Some(&title));
    }

    fn setup_actions(&self) {
        let calculate_bom_action = gio::ActionEntry::builder("calculate-bom")
            .activate(glib::clone!(@weak self as window => move |_, _, _| {
                window.calculate_bom();
            }))
            .build();

        self.add_action_entries([calculate_bom_action]);
    }

    fn calculate_bom(&self) {
        let imp = self.imp();
        if let Some(project) = &*imp.project.borrow() {
            let bom_engine = BomEngine::new();
            match bom_engine.calculate_bom(project) {
                Ok(bom) => {
                    tracing::info!(
                        "BOM calculated: {} items, total cost: €{:.2}",
                        bom.items.len(),
                        bom.total_cost
                    );

                    // Show BOM results in a dialog
                    let dialog = gtk::MessageDialog::builder()
                        .transient_for(self)
                        .modal(true)
                        .message_type(gtk::MessageType::Info)
                        .text("BOM Calculation Complete")
                        .secondary_text(&format!(
                            "Calculated {} items\nTotal cost: €{:.2}",
                            bom.items.len(),
                            bom.total_cost
                        ))
                        .build();

                    dialog.present();
                }
                Err(e) => {
                    tracing::error!("BOM calculation failed: {}", e);

                    let dialog = gtk::MessageDialog::builder()
                        .transient_for(self)
                        .modal(true)
                        .message_type(gtk::MessageType::Error)
                        .text("BOM Calculation Failed")
                        .secondary_text(&format!("Error: {}", e))
                        .build();

                    dialog.present();
                }
            }
        }
    }
}

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(string = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <interface>
          <template class="NovaWindow" parent="AdwApplicationWindow">
            <property name="width-request">800</property>
            <property name="height-request">600</property>
            <property name="title">Nova Design</property>
            <child>
              <object class="AdwHeaderBar" id="header_bar">
                <child type="start">
                  <object class="GtkMenuButton">
                    <property name="icon-name">open-menu-symbolic</property>
                    <property name="menu-model">primary_menu</property>
                  </object>
                </child>
                <child type="end">
                  <object class="GtkButton">
                    <property name="icon-name">applications-engineering-symbolic</property>
                    <property name="tooltip-text">Calculate BOM</property>
                    <property name="action-name">win.calculate-bom</property>
                  </object>
                </child>
              </object>
            </child>
            <child>
              <object class="NovaWorkspaceView" id="workspace">
                <property name="hexpand">true</property>
                <property name="vexpand">true</property>
              </object>
            </child>
          </template>
          <menu id="primary_menu">
            <section>
              <item>
                <attribute name="label">New Project</attribute>
                <attribute name="action">app.new-project</attribute>
              </item>
              <item>
                <attribute name="label">Open Project</attribute>
                <attribute name="action">app.open-project</attribute>
              </item>
              <item>
                <attribute name="label">Save Project</attribute>
                <attribute name="action">app.save-project</attribute>
              </item>
            </section>
            <section>
              <item>
                <attribute name="label">About Nova Design</attribute>
                <attribute name="action">app.about</attribute>
              </item>
              <item>
                <attribute name="label">Quit</attribute>
                <attribute name="action">app.quit</attribute>
              </item>
            </section>
          </menu>
        </interface>
    "#)]
    pub struct NovaWindow {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub workspace: TemplateChild<WorkspaceView>,

        pub project: RefCell<Option<Project>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NovaWindow {
        const NAME: &'static str = "NovaWindow";
        type Type = super::NovaWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NovaWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_actions();
            obj.update_title();
        }
    }

    impl WidgetImpl for NovaWindow {}
    impl WindowImpl for NovaWindow {}
    impl ApplicationWindowImpl for NovaWindow {}
    impl AdwApplicationWindowImpl for NovaWindow {}
}
