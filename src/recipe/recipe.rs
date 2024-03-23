use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::{gio, glib};

#[derive(Properties, Default)]
#[properties(wrapper_type = super::Recipe)]
pub struct Recipe {
    #[property(get, set)]
    pub name: RefCell<String>,
    #[property(get, set)]
    pub ingredientlist: OnceCell<gio::ListStore>,
}

#[glib::object_subclass]
impl ObjectSubclass for Recipe {
    const NAME: &'static str = "RecipeObject";
    type Type = super::Recipe;
}

#[glib::derived_properties]
impl ObjectImpl for Recipe {}
