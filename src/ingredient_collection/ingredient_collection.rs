use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::{gio, glib};

#[derive(Properties, Default)]
#[properties(wrapper_type = super::IngredientCollection)]
pub struct IngredientCollection {
    #[property(get, set)]
    pub name: RefCell<String>,
    #[property(get, set)]
    pub ingredientlist: OnceCell<gio::ListStore>,
}

#[glib::object_subclass]
impl ObjectSubclass for IngredientCollection {
    const NAME: &'static str = "IngredientCollectionObject";
    type Type = super::IngredientCollection;
}

#[glib::derived_properties]
impl ObjectImpl for IngredientCollection {}
