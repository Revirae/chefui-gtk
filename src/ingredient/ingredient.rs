use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use adw::subclass::prelude::*;
use super::IngredientData;

pub trait IngredientImpl {
 // FoodPortion(Food, usize),
}

#[derive(Properties, Default)]
#[properties(wrapper_type = super::IngredientObject)]
pub struct IngredientObject {
    #[property(name = "kind", get, set, type = String, member = kind)]
    #[property(name = "name", get, set, type = String, member = name)]
    #[property(name = "amount", get, set, type = u32, member = amount)]

    pub data: RefCell<IngredientData>,
}

#[glib::object_subclass]
impl adw::glib::subclass::prelude::ObjectSubclass for IngredientObject {
    const NAME: &'static str = "IngredientObject";
    type Type = super::IngredientObject;
}

#[glib::derived_properties]
impl adw::glib::subclass::prelude::ObjectImpl for IngredientObject {}

