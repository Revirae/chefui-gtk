// mod food;
mod window;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio::{ActionGroup, ActionMap, ListStore};
use gtk::glib::Object;
use gtk::glib::{clone, wrapper};
use gtk::{glib, Button};
use gtk::{
    Accessible, Box, Buildable, ConstraintTarget, CustomFilter, FilterListModel, Native,
    NoSelection, ShortcutManager,
};

use crate::action::Action;
use crate::food_collection::FoodCollection;
use crate::cuisine::{Cuisine, FoodStore};
use crate::food::FoodObject;

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
        Object::builder().property("application", app).build()
    }

    pub fn foodlist(&self) -> ListStore {
        let app = self.imp();
        app.main_fc
            .borrow()
            .clone()
            .expect("current collection not set")
            .foodlist()
    }

    pub fn collections(&self) -> ListStore {
        let app = self.imp();
        app.food_collections
            .get()
            .expect("`collections` should be set in `setup_collections`")
            .clone()
    }

    fn clear_form(&self) {
        let app = self.imp();

        for w in [
            &app.entry_name,
            &app.entry_brand,
            &app.entry_cost,
            &app.entry_weight,
            &app.entry_volume,
        ] {
            w.set_text("");
        }

        // let _ = app.food_mode.set(false);
        app.food_mode.replace(false);
        app.button_submit.set_label("registrar");
    }
    fn new_food(&self) {
        let app = self.imp();

        let name = app.entry_name.text().to_string();
        if name.is_empty() {
            return;
        }
        app.entry_name.set_text("");

        let brand = app.entry_brand.text().to_string();
        app.entry_brand.set_text("");

        let cost: String = app.entry_cost.text().into();
        let cost: f32 = cost.parse().unwrap_or_default();
        let cost: u32 = (cost * 1.) as u32;

        let weight: String = app.entry_weight.text().into();
        let weight: u32 = weight.parse().unwrap_or_default();

        let volume: String = app.entry_volume.text().into();
        let volume: u32 = volume.parse().unwrap_or_default();

        app.entry_cost.set_text("");

        let new_food = FoodObject::new(
            name.clone(),
            brand.clone(),
            cost,
            weight,
            volume
            // true,
            // false
        );

        // let is_updating = *app.food_mode.get().unwrap();
        let is_updating: bool = app.food_mode.clone().into_inner();
        dbg!(is_updating);
        if is_updating {
            let target = app.update_key.clone().into_inner().unwrap();
            if let Some(i) = self.foodlist().find(&target) {
                if let Some(obj) = self.foodlist().item(i) {
                               
                    let food: &FoodObject = obj
                        .downcast_ref::<FoodObject>()
                        .expect("todo");
                    let key = food.name();
                    food.set_name(name);
                    food.set_brand(brand);
                    food.set_cost(cost);
                    food.set_weight(weight);
                    food.set_volume(volume);
                    // food.set_mustupdate(true);
                    app.commits
                        .borrow_mut()
                        .push(Action::Update(key, food.data()));
                }
            }
            // let _ = app.food_mode.set(false);
            // app.food_mode = false;
            app.food_mode.replace(false);
            app.button_submit.set_label("registrar");//todo
            
        } else {
            self.foodlist().append(&new_food);
        }
    }

    fn update_food(&self, obj: &FoodObject) {
        let app = self.imp();

        // let _ = app.update_key.set(obj.clone());
        app.update_key.replace(Some(obj.clone()));

        app.entry_name.set_text(&obj.name());
        app.entry_brand.set_text(&obj.brand());

        let cost = format!("{}", obj.cost());
        app.entry_cost.set_text(&cost);

        let weight = format!("{}", obj.weight());
        app.entry_weight.set_text(&weight);

        let volume = format!("{}", obj.volume());
        app.entry_volume.set_text(&volume);

        // let _ = app.food_mode.set(true);
        app.food_mode.replace(true);
        // app.food_mode = true;
        dbg!(app.food_mode.borrow());
        app.button_submit.set_label("atualizar");
    }

    fn new_collection(&self) {
        let foodlist = ListStore::new::<FoodObject>();

        let name = "test".to_owned();
        let foodlist = FoodCollection::new(&name, foodlist);

        self.collections().append(&foodlist);
        self.set_current_collection(foodlist);
    }

    fn create_collection_row(&self, obj: &FoodCollection) -> adw::ActionRow {
        let row = adw::ActionRow::new();

        obj.bind_property("name", &row, "title")
            .sync_create()
            .build();

        row
    }

    fn setup_collections(&self) {
        let app = self.imp();
        let collection = self.foodlist();

        app.food_list.bind_model(
            Some(&collection),
            clone!(
                @weak self as window => @default-panic, move |food| {
                let food_object = food.downcast_ref().expect("the object should be of type `CollectionObject`");
                let row = window.create_food_row(food_object);
                row.upcast()
            })
        )
    }

    fn create_food_row(&self, food_object: &FoodObject) -> adw::ActionRow {
        let content = Box::builder()
            .hexpand(true)
            .halign(gtk::Align::Start)
            .build();

        let widget_cost = gtk::Box::builder()
            .halign(gtk::Align::Center)
            .width_request(48)
            .build();
        let widget_cost_row = adw::ActionRow::builder()
            .subtitle("$")
            // .halign(gtk::Align::Start)
            .build();
        widget_cost.append(&widget_cost_row);
        content.append(&widget_cost);

        let widget_weight = Box::builder().width_request(150).build();
        let widget_weight_row = adw::ActionRow::builder().subtitle("grama").build();
        widget_weight.append(&widget_weight_row);
        content.append(&widget_weight);

        let widget_volume = Box::builder().halign(gtk::Align::End).build();
        let widget_volume_row = adw::ActionRow::builder()
            .subtitle("ml")
            // .halign(gtk::Align::Center)
            .build();
        widget_volume.append(&widget_volume_row);
        content.append(&widget_volume);

        let delete_button = Button::builder().build();

        let row = adw::ActionRow::builder()
            // .halign(gtk::Align::BaselineFill)
            .build();
        row.add_prefix(&content);
        row.add_suffix(&delete_button);

        food_object
            .bind_property("name", &row, "title")
            .sync_create()
            .bidirectional()
            .build();

        food_object
            .bind_property("brand", &row, "subtitle")
            .sync_create()
            .build();

        food_object
            .bind_property("cost", &widget_cost_row, "title")
            .sync_create()
            .build();

        food_object
            .bind_property("weight", &widget_weight_row, "title")
            .sync_create()
            .build();

        food_object
            .bind_property("volume", &widget_volume_row, "title")
            .sync_create()
            .build();

        row
    }

    fn set_current_collection(&self, collection: FoodCollection) {
        let app = self.imp();

        let foodlist = collection.foodlist();
        let filter_model = FilterListModel::new(Some(foodlist.clone()), self.filter());
        let selection_model = NoSelection::new(Some(filter_model));

        app.food_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |obj| {
                let food_object = obj
                    .downcast_ref()
                    .expect("the object should be
                        of type `FoodObject`");
                let row = window.create_food_row(food_object);
                row.upcast()
            }),
        );

        app.main_fc.replace(Some(collection));
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

    pub fn setup_cuisine(&self) {
        let app = self.imp();
        app.cuisine
            .set(Cuisine {})
            .expect("failed to set up cuisine");
    }

    pub fn setup_database(&self) {
        let app = self.imp();

        let cuisine = app.cuisine
            .get()
            .expect("trying to set up database with bad cuisine");

        let food_store = FoodStore::load_or_init(&cuisine).expect("failed do load or init store");
        app.food_store.set(food_store).expect("failed to setup food_store");
    }

    pub fn load_database(&self) {
        let app = self.imp();

        let food_store = app.food_store.get().expect(
            "failed to get store to load database, 
            was database set up before loading?",
        );

        let cuisine = app.cuisine.get().unwrap();

        let food_collections = ListStore::new::<FoodCollection>();

        if let Ok(data) = food_store.get_food(&cuisine) {
            let collection = FoodCollection::from_collection_data(data);
            // self.collections()
            food_collections.append(&collection);
            self.set_current_collection(collection);
        }

        app.food_collections
            .set(food_collections.clone())
            .expect("failed to set food collections");
    }

    pub fn setup(&self) {
        let app = self.imp();

        // app.stack.set_visible_child_name("main");
        // app.food_mode.set(false).expect("todo");
        app.food_mode.replace(false);

        app.food_list
            .connect_row_selected(clone!(@weak self as window => move |_, row| {
                let index = row.unwrap().index();
                let selected_food = window.foodlist()
                    .item(index as u32)
                    .expect("there needs to be an object at this point")
                    .downcast::<FoodObject>()
                    .expect("the object needs to be a `FoodObject`");
                // todo!()

                window.update_food(&selected_food);
                // println!("...");

            }));

        app.button_submit
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.new_food();
            }));

        app.button_clear
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.clear_form();
            }));
    }
}
