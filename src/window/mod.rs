mod window;

use adw::ActionRow;
use gio::{ActionGroup, ActionMap};
use glib::Object;
use gtk::glib::clone;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::{
    gio, glib, pango, FilterListModel, Label,
    ListBoxRow, NoSelection,
};
use gtk::{prelude::*, CustomFilter};
use gtk::{
    Accessible, Buildable, ConstraintTarget, Native,
    ShortcutManager,
};

use crate::food::FoodObject;
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

    pub fn foodcollections(&self) -> gio::ListStore {
        let app = self.imp();
        app.food_collections
            .get()
            .expect("`collections` should be set in `setup_collections`")
            .clone()
    }

    fn new_food(&self) {
        let app = self.imp();

        let name = app.entry_name.text().to_string();
        if name.is_empty() {
            return;
        }
        app.entry_name.set_text("");

        let brand =
            app.entry_brand.text().to_string();
        app.entry_brand.set_text("");

        let new_food =
            FoodObject::new(name, brand, 0, 0, 0);
        self.foodlist().append(&new_food);
    }

    fn new_collection(&self) {
        let foodlist =
            gio::ListStore::new::<FoodObject>();

        let name = "test".to_owned();
        let foodlist =
            FoodCollection::new(&name, foodlist);

        self.foodcollections().append(&foodlist);
        self.set_current_collection(foodlist);
    }

    fn create_collection_row(
        &self,
        obj: &FoodCollection,
    ) -> adw::ActionRow {
        // let label_name = Label::builder()
        // .ellipsize(pango::EllipsizeMode::End)
        // .xalign(0.0)
        // .build();
        let row = adw::ActionRow::new();

        obj.bind_property("name", &row, "title")
            .sync_create()
            .build();

        row
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

    fn create_food_row(
        &self,
        food_object: &FoodObject,
    ) -> adw::ActionRow {
        let row = ActionRow::builder().build();

        food_object
            .bind_property("name", &row, "title")
            .sync_create()
            .build();

        row
    }

    fn set_current_collection(
        &self,
        collection: FoodCollection,
    ) {
        let app = self.imp();

        let foodlist = collection.foodlist();
        let filter_model = FilterListModel::new(
            Some(foodlist.clone()),
            self.filter(),
        );
        let selection_model =
            NoSelection::new(Some(filter_model));

        app.food_list.bind_model(Some(&selection_model),clone!(@weak self as window => @default-panic, move |obj| {
            let food_object = obj.downcast_ref().expect("the object should be of type `FoodObject`");
            let row = window.create_food_row(food_object);
            row.upcast()
        }));

        app.current_fc.replace(Some(collection));
    }

    fn filter(&self) -> Option<CustomFilter> {
        // let filter = CustomFilter::new()

        // let _filter_x = CustomFilter::new(|obj| {
        // let food_object = obj
        // .downcast_ref::<FoodObject>()
        // .expect("the object needs to be of type `FoodObject`");
        // true//todo
        // });

        // match filter_state.as_str() {
        //     "All" => None,
        //     _ => unreachable!(),
        // }
        None
    }

    pub fn setup(&self) {
        let app = self.imp();

        // ActionGroupExt::activate_action(
        //     self,
        //     "chef.new-collection",
        //     None,
        // );
        // spawn_blocking(self.new_collection);
        // self.add_action()
        // println!("{:?}", self.list_actions());
        // .expect(
        // "failed to create initial collection",
        // );

        self.new_collection();
        app.stack.set_visible_child_name("main");

        app.button_submit.connect_clicked(
            clone!(@weak self as window => move |_| {
                window.new_food();
            }),
        );
    }
}
