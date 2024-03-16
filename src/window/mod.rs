mod window;

use gio::{ActionGroup, ActionMap};
use glib::Object;
use gtk::{gio, glib};
use gtk::{
    Accessible, Buildable, ConstraintTarget, Native,
    ShortcutManager,
};

glib::wrapper! {
    pub struct ChefApp(ObjectSubclass<window::ChefApp>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
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
}
