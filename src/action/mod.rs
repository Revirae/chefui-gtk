mod action;

use glib::Object;
use gtk::glib::{
    self, subclass::types::ObjectSubclassIsExt//, subclass::types::ObjectSubclassIsExt,
};
use serde::{Deserialize, Serialize};

use crate::food::Food;

glib::wrapper! {
    pub struct ActionObject(ObjectSubclass<action::ActionObject>);
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Action {
    Create(Food), Update(String, Food), Delete,
    #[default] NoAction
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActionData {
    pub name: String,
}

impl ActionObject {
    pub fn new(name: &str) -> Self {
        Object::builder()
            .property("name", name)
            .build()
    }
    pub fn data(&self) -> ActionData {
        self.imp().data.borrow().clone()
    }
    pub fn from_data(data: ActionData) -> Self {
        Self::new(data.name.as_str())
    }
}
