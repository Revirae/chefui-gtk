mod window;

use gio::{ActionGroup, ActionMap};
use glib::Object;
use gtk::glib::clone;
use gtk::glib::object::{Cast, ObjectExt};
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::{gio, glib, pango, Label, ListBoxRow};
use gtk::{
    Accessible, Buildable, ConstraintTarget, Native,
    ShortcutManager,
};

use crate::food_collection::FoodCollection;

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

    pub fn foodlist(&self) -> gio::ListStore {
        let app = self.imp();
        app.current_fc
            .borrow()
            .clone()
            .expect("current collection not set")
            .foodlist()
    }

    fn create_collection_row(
        &self,
        obj: &FoodCollection,
    ) -> ListBoxRow {
        let label_name = Label::builder()
            .ellipsize(pango::EllipsizeMode::End)
            .xalign(0.0)
            .build();
        obj.bind_property(
            "name",
            &label_name,
            "label",
        )
        .sync_create()
        .build();

        ListBoxRow::builder()
            .child(&label_name)
            .build()
    }

    fn setup_collections(&self) {
        let app = self.imp();
        let collections =
            gio::ListStore::new::<FoodCollection>();
        app.food_collections
            .set(collections.clone())
            .expect("failed to set collections");
        app.food_list.bind_model(
            Some(&collections),
            clone!(@weak self as window => @default-panic, move |food| {
                let collection_object = food.downcast_ref().expect("the object should be of type `CollectionObject`");
                let row = window.create_collection_row(collection_object);
                row.upcast()
            })
        )
    }

    pub fn setup(&self) {
        let app = self.imp();

        app.stack.set_visible_child_name("main");

        // app.entry_name
        // .add_prefix(&Label::new(Some(">>")));
    }
}
