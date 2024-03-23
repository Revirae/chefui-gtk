use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
// use gtk::subclass::prelude::*;
use adw::subclass::prelude::*;
// use super::Action;
use super::Food;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::FoodObject)]
pub struct FoodObject {
    #[property(name = "name", get, set, type = String, member = name)]
    #[property(name = "brand", get, set, type = String, member = brand)]
    #[property(name = "cost", get, set, type = u32, member = cost)]
    #[property(name = "weight", get, set, type = u32, member = weight)]
    #[property(name = "volume", get, set, type = u32, member = volume)]
    // #[property(name = "action", get, set, type = Action, member = action)]

    pub data: RefCell<Food>,
}

#[glib::object_subclass]
impl adw::glib::subclass::prelude::ObjectSubclass for FoodObject {
    const NAME: &'static str = "FoodObject";
    type Type = super::FoodObject;
}

#[glib::derived_properties]
impl adw::glib::subclass::prelude::ObjectImpl for FoodObject {}

