use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::glib::subclass::InitializingObject;

#[derive(gtk::CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/chef/chef.ui")]
pub struct ChefApp {
    // #[template_child]
    // pub button: TemplateChild<gtk::Button>,
}

#[gtk::template_callbacks]
impl ChefApp {
    #[template_callback]
    fn exit_app(_button: &gtk::Button) {}
}

#[glib::object_subclass]
impl ObjectSubclass for ChefApp {
    const NAME: &'static str = "ChefApp";
    type Type = super::ChefApp;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ChefApp {
    fn constructed(&self) {
        self.parent_constructed();
        // let obj = self.obj();
        // obj.setup_settings();
        // obj.setup_callbacks();
        // obj.setup_users();
        // obj.setup_actions();
    }
    fn dispose(&self) {
        while let Some(child) =
            self.obj().first_child()
        {
            child.unparent();
        }
    }
}

impl WidgetImpl for ChefApp {}

impl AdwWindowImpl for ChefApp {}
impl WindowImpl for ChefApp {}

impl AdwApplicationWindowImpl for ChefApp {}
impl ApplicationWindowImpl for ChefApp {}
