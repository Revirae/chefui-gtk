
use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::{gio, glib};

#[derive(Properties, Default)]
#[properties(wrapper_type = super::RecipeCollection)]
pub struct RecipeCollection {
    #[property(get, set)]
    pub name: RefCell<String>,
    #[property(get, set)]
    pub ingredientlist: OnceCell<gio::ListStore>,
}

#[glib::object_subclass]
impl ObjectSubclass for RecipeCollection {
    const NAME: &'static str = "RecipeCollectionObject";
    type Type = super::RecipeCollection;
}

#[glib::derived_properties]
impl ObjectImpl for RecipeCollection {}
