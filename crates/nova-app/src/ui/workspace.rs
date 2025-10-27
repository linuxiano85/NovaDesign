use adw::prelude::*;
use glib::Object;
use gtk4 as gtk;
use std::cell::RefCell;

use nova_core::{Discipline, Phase, Project};

glib::wrapper! {
    pub struct WorkspaceView(ObjectSubclass<imp::WorkspaceView>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WorkspaceView {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_project(&self, project: Option<&Project>) {
        let imp = self.imp();
        imp.project.replace(project.cloned());
        self.update_project_view();
    }

    fn update_project_view(&self) {
        let imp = self.imp();

        // Clear existing content
        while let Some(child) = imp.content_area.first_child() {
            imp.content_area.remove(&child);
        }

        if let Some(project) = &*imp.project.borrow() {
            // Create project overview
            let overview_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
            overview_box.set_margin_top(12);
            overview_box.set_margin_bottom(12);
            overview_box.set_margin_start(12);
            overview_box.set_margin_end(12);

            // Project info
            let title_label = gtk::Label::new(Some(&project.name));
            title_label.add_css_class("title-1");
            overview_box.append(&title_label);

            let desc_label = gtk::Label::new(Some(&project.description));
            desc_label.add_css_class("dim-label");
            overview_box.append(&desc_label);

            // Buildings list
            if !project.buildings.is_empty() {
                let buildings_label = gtk::Label::new(Some("Buildings:"));
                buildings_label.add_css_class("title-3");
                buildings_label.set_halign(gtk::Align::Start);
                buildings_label.set_margin_top(12);
                overview_box.append(&buildings_label);

                for building in &project.buildings {
                    let building_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
                    building_box.set_margin_start(12);

                    let building_label = gtk::Label::new(Some(&building.name));
                    building_label.add_css_class("title-4");
                    building_label.set_halign(gtk::Align::Start);
                    building_box.append(&building_label);

                    let address_label = gtk::Label::new(Some(&building.address));
                    address_label.add_css_class("dim-label");
                    address_label.set_halign(gtk::Align::Start);
                    building_box.append(&address_label);

                    // Floors
                    for floor in &building.floors {
                        let floor_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                        floor_box.set_margin_start(12);

                        let floor_label = gtk::Label::new(Some(&format!("Floor: {}", floor.name)));
                        floor_label.set_halign(gtk::Align::Start);
                        floor_box.append(&floor_label);

                        let stats_label = gtk::Label::new(Some(&format!(
                            "Walls: {}, Components: {}",
                            floor.walls.len(),
                            floor.components.len()
                        )));
                        stats_label.add_css_class("dim-label");
                        stats_label.set_halign(gtk::Align::Start);
                        floor_box.append(&stats_label);

                        building_box.append(&floor_box);
                    }

                    overview_box.append(&building_box);
                }
            } else {
                let empty_label = gtk::Label::new(Some(
                    "No buildings yet. Start by adding a building to your project.",
                ));
                empty_label.add_css_class("dim-label");
                empty_label.set_halign(gtk::Align::Start);
                empty_label.set_margin_top(12);
                overview_box.append(&empty_label);
            }

            imp.content_area.append(&overview_box);
        } else {
            // No project loaded
            let welcome_box = gtk::Box::new(gtk::Orientation::Vertical, 24);
            welcome_box.set_halign(gtk::Align::Center);
            welcome_box.set_valign(gtk::Align::Center);
            welcome_box.set_hexpand(true);
            welcome_box.set_vexpand(true);

            let welcome_label = gtk::Label::new(Some("Welcome to Nova Design"));
            welcome_label.add_css_class("title-1");
            welcome_box.append(&welcome_label);

            let subtitle_label = gtk::Label::new(Some("Linux CAD/BIM light for trades"));
            subtitle_label.add_css_class("title-3");
            subtitle_label.add_css_class("dim-label");
            welcome_box.append(&subtitle_label);

            let instructions_label = gtk::Label::new(Some(
                "Create a new project or open an existing one to get started.",
            ));
            instructions_label.add_css_class("dim-label");
            welcome_box.append(&instructions_label);

            // Quick actions
            let actions_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
            actions_box.set_halign(gtk::Align::Center);
            actions_box.set_margin_top(24);

            let new_button = gtk::Button::with_label("New Project");
            new_button.add_css_class("suggested-action");
            new_button.set_action_name(Some("app.new-project"));
            actions_box.append(&new_button);

            let open_button = gtk::Button::with_label("Open Project");
            open_button.set_action_name(Some("app.open-project"));
            actions_box.append(&open_button);

            welcome_box.append(&actions_box);
            imp.content_area.append(&welcome_box);
        }
    }
}

impl Default for WorkspaceView {
    fn default() -> Self {
        Self::new()
    }
}

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(string = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <interface>
          <template class="NovaWorkspaceView" parent="GtkBox">
            <property name="orientation">horizontal</property>
            <property name="spacing">0</property>
            <child>
              <object class="GtkBox" id="sidebar">
                <property name="orientation">vertical</property>
                <property name="width-request">200</property>
                <style>
                  <class name="sidebar"/>
                </style>
                <child>
                  <object class="AdwHeaderBar">
                    <property name="show-end-title-buttons">false</property>
                    <property name="show-start-title-buttons">false</property>
                    <child type="title">
                      <object class="GtkLabel">
                        <property name="label">Layers</property>
                        <style>
                          <class name="title-4"/>
                        </style>
                      </object>
                    </child>
                  </object>
                </child>
                <child>
                  <object class="GtkScrolledWindow">
                    <property name="hscrollbar-policy">never</property>
                    <property name="vexpand">true</property>
                    <child>
                      <object class="GtkListBox" id="layers_list">
                        <property name="selection-mode">none</property>
                        <style>
                          <class name="navigation-sidebar"/>
                        </style>
                      </object>
                    </child>
                  </object>
                </child>
              </object>
            </child>
            <child>
              <object class="GtkSeparator">
                <property name="orientation">vertical</property>
              </object>
            </child>
            <child>
              <object class="GtkScrolledWindow" id="content_area">
                <property name="hexpand">true</property>
                <property name="vexpand">true</property>
              </object>
            </child>
          </template>
        </interface>
    "#)]
    pub struct WorkspaceView {
        #[template_child]
        pub sidebar: TemplateChild<gtk::Box>,
        #[template_child]
        pub layers_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub content_area: TemplateChild<gtk::ScrolledWindow>,

        pub project: RefCell<Option<Project>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WorkspaceView {
        const NAME: &'static str = "NovaWorkspaceView";
        type Type = super::WorkspaceView;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for WorkspaceView {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.update_project_view();
        }
    }

    impl WidgetImpl for WorkspaceView {}
    impl BoxImpl for WorkspaceView {}
}
