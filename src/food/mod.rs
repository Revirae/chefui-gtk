mod food;

// use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib::{
    self, subclass::types::ObjectSubclassIsExt,
};
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct FoodObject(ObjectSubclass<food::FoodObject>);
}

// let a = ObjectSubclass
impl FoodObject {
    pub fn new(
        name: String,
        brand: String,
        cost: u32,
        weight: u32,
        volume: u32,
    ) -> Self {
        Object::builder()
            .property("name", name)
            .property("brand", brand)
            .property("cost", cost)
            .property("weight", weight)
            .property("volume", volume)
            .build()
    }
    pub fn data(&self) -> Food {
        self.imp().data.borrow().clone()
    }
    pub fn from_data(data: Food) -> Self {
        Self::new(
            data.name,
            data.brand,
            data.cost,
            data.weight,
            data.volume,
        )
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Food {
    pub name: String,
    pub brand: String,
    pub cost: u32,
    pub weight: u32,
    pub volume: u32,
}
