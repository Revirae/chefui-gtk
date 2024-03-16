mod window;

use gio::{ActionGroup, ActionMap};
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::{gio, glib};
use gtk::{
    Accessible, Buildable, ConstraintTarget, Native,
    ShortcutManager,
};

glib::wrapper! {
    pub struct ChefApp(ObjectSubclass<window::ChefApp>)
        @extends adw::ApplicationWindow, adw::Window,
                 gtk::Window, gtk::Widget,
        @implements ActionGroup, ActionMap, Accessible, Buildable,
                    ConstraintTarget, ShortcutManager, Native,
                    gtk::Root;
}

impl ChefApp {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder()
            .property("application", app)
            .build()
    }

    pub fn setup(&self) {
        let app = self.imp();

        // app.main_stack
        // .set_visible_child_name("page0");
    }
}
