use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
// use gtk::subclass::prelude::*;
use adw::subclass::prelude::*;

use super::ActionData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::ActionObject)]
pub struct ActionObject {
    #[property(name = "name", get, set, type = String, member = name)]

    pub data: RefCell<ActionData>,
}

#[glib::object_subclass]
impl adw::glib::subclass::prelude::ObjectSubclass for ActionObject {
    const NAME: &'static str = "ActionObject";
    type Type = super::ActionObject;
}

#[glib::derived_properties]
impl adw::glib::subclass::prelude::ObjectImpl for ActionObject {}
