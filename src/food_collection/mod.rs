mod food_collection;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, Object},
};
use serde::{Deserialize, Serialize};

use crate::food::{Food, FoodObject};

glib::wrapper! {
    pub struct FoodCollection(ObjectSubclass<food_collection::FoodCollection>);
}

impl FoodCollection {
    pub fn new(
        name: &str,
        foodlist: gio::ListStore,
    ) -> Self {
        Object::builder()
            .property("name", name)
            .property("foodlist", foodlist)
            .build()
    }
    pub fn to_collection_data(
        &self,
    ) -> FoodCollectionData {
        let name = self.imp().name.borrow().clone();
        let foodlist = self
            .foodlist()
            .iter::<FoodObject>()
            .filter_map(Result::ok)
            .map(|food_object| food_object.data())
            .collect();
        FoodCollectionData { name, foodlist }
    }
    pub fn from_collection_data(
        data: FoodCollectionData,
    ) -> Self {
        let name = data.name;
        let food_to_extend: Vec<FoodObject> = data
            .foodlist
            .into_iter()
            .map(FoodObject::from_data)
            .collect();
        let foodlist =
            gio::ListStore::new::<FoodObject>();
        foodlist.extend_from_slice(&food_to_extend);

        Self::new(&name, foodlist)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct FoodCollectionData {
    pub name: String,
    pub foodlist: Vec<Food>,
}
