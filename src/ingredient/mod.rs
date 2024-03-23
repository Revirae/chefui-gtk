mod ingredient;

use glib::Object;
use gtk::glib::{
    self, subclass::types::ObjectSubclassIsExt,
};
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct IngredientObject(ObjectSubclass<ingredient::IngredientObject>);
}

// pub enum IngredientKind {
    // FoodPortion(Food, usize),
// }

impl IngredientObject {
    pub fn new(
        name: String,
    ) -> Self {
        Object::builder()
            .property("name", name)
            .build()
    }
    pub fn data(&self) -> IngredientData {
        self.imp().data.borrow().clone()
    }
    pub fn from_data(data: IngredientData ) -> Self {
        Self::new(
            data.name,
        )
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IngredientData {
    pub name: String,
}
