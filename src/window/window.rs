use crate::recipe_collection::RecipeCollection;
use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio::ListStore;
use gtk::glib::subclass::InitializingObject;
use gtk::{glib, Button, ListBox};

use crate::action::Action;
use crate::food_collection::FoodCollection;
use crate::cuisine::{Cuisine, FoodStore, IngredientStore};
use crate::food::FoodObject;

// pub enum FoodMode { }

#[derive(gtk::CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/chef/chef.xml")]
pub struct ChefApp{
    #[template_child]
    pub main_stack: TemplateChild<adw::ViewStack>,
    // #[template_child]
    // pub recipe_list: TemplateChild<ListBox>,
    #[template_child]
    pub food_list: TemplateChild<ListBox>,
    #[template_child]
    pub entry_name: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub entry_brand: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub entry_cost: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub entry_weight: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub entry_volume: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub button_submit: TemplateChild<Button>,
    #[template_child]
    pub button_clear: TemplateChild<Button>,
    //----
    pub food_collections: OnceCell<ListStore>,
    pub ingredient_collections: OnceCell<ListStore>,
    pub main_fc: RefCell<Option<FoodCollection>>,
    pub main_rc: RefCell<Option<RecipeCollection>>,
    //-----
    pub cuisine: OnceCell<Cuisine>,
    pub food_store: OnceCell<FoodStore>,
    pub recipe_store: OnceCell<IngredientStore>,
    //-----
    pub food_mode: RefCell<bool>,
    pub update_key: RefCell<Option<FoodObject>>,

    pub commits: RefCell<Vec<Action>>,
}

#[gtk::template_callbacks]
impl ChefApp {
    #[template_callback]
    fn exit_app(_button: &gtk::Button) {}

    // #[template_callback]
    // fn
}

#[glib::object_subclass]
impl ObjectSubclass for ChefApp {
    const NAME: &'static str = "ChefApp";
    type Type = super::ChefApp;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();

        klass.install_action_async(
            "chef.new-collection",
            None,
            |window, _, _| async move {
                window.new_collection();
            },
        );
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ChefApp {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // self.update_mode.set(false).unwrap();
        obj.setup_cuisine();
        obj.setup_database();
        obj.load_database();
        obj.setup_collections();
        obj.setup();
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

impl AdwWindowImpl for ChefApp {
    
}
impl WindowImpl for ChefApp {
    fn close_request(&self) -> glib::Propagation {
        let cuisine = self.cuisine.get().unwrap();
        let food_store = self
            .food_store
            .get()
            .expect("failed to retrieve store");

        dbg!("supposed to be working main_fc");
        if let Some(foodlist) =
            self.main_fc.clone().into_inner()
        {
            let _ = food_store
                .send_food(
                    &cuisine,
                    foodlist.to_collection_data(),
                    &self.commits.borrow()
                )
                .is_err_and(|e| {
                    eprintln!("{:?}", e);
                    true
                });
        }
        self.parent_close_request()
    }
}

impl AdwApplicationWindowImpl for ChefApp {}
impl ApplicationWindowImpl for ChefApp {}
