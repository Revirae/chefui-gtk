mod window;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio::{ActionGroup, ActionMap, ListStore};
use gtk::glib::Object;
use gtk::glib::{clone, wrapper};
// use gtk::prelude::*;
use gtk::{glib, Label};
use gtk::{
    Accessible, Box, Buildable, ConstraintTarget,
    CustomFilter, FilterListModel, Native,
    NoSelection, ShortcutManager,
};

use crate::food::FoodObject;
use crate::food_collection::FoodCollection;

wrapper! {
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

    pub fn foodlist(&self) -> ListStore {
        let app = self.imp();
        app.current_fc
            .borrow()
            .clone()
            .expect("current collection not set")
            .foodlist()
    }

    pub fn foodcollections(&self) -> ListStore {
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

        let cost: String =
            app.entry_cost.text().into();
        let cost: f32 =
            cost.parse().unwrap_or_default();
        let cost: u32 = (cost * 100.) as u32;

        let weight: String =
            app.entry_weight.text().into();
        let weight: u32 =
            weight.parse().unwrap_or_default();

        let volume: String =
            app.entry_volume.text().into();
        let volume: u32 =
            volume.parse().unwrap_or_default();

        app.entry_cost.set_text("");

        let new_food = FoodObject::new(
            name, brand, cost, weight, volume,
        );
        self.foodlist().append(&new_food);
    }

    fn new_collection(&self) {
        let foodlist = ListStore::new::<FoodObject>();

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
            ListStore::new::<FoodCollection>();
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
        let content = Box::builder().build();

        let label_cost = Label::builder().build();
        content.append(&label_cost);

        let label_weight = Label::builder().build();
        content.append(&label_weight);

        let label_volume = Label::builder().build();
        content.append(&label_volume);

        let row = adw::ActionRow::builder().build();
        row.add_suffix(&content);

        food_object
            .bind_property("name", &row, "title")
            .sync_create()
            .build();

        food_object
            .bind_property("brand", &row, "subtitle")
            .sync_create()
            .build();

        food_object
            .bind_property(
                "cost",
                &label_cost,
                "label",
            )
            .sync_create()
            .build();

        food_object
            .bind_property(
                "weight",
                &label_weight,
                "label",
            )
            .sync_create()
            .build();

        food_object
            .bind_property(
                "volume",
                &label_volume,
                "label",
            )
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
