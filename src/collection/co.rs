use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::{gio, glib};

#[derive(Properties, Default)]
#[properties(wrapper_type = super::FoodCollection)]
pub struct FoodCollection {
    #[property(get, set)]
    pub name: RefCell<String>,
    #[property(get, set)]
    pub foodlist: OnceCell<gio::ListStore>,
}

#[glib::object_subclass]
impl ObjectSubclass for FoodCollection {
    const NAME: &'static str = "FoodCollectionObject";
    type Type = super::FoodCollection;
}

#[glib::derived_properties]
impl ObjectImpl for FoodCollection {}
