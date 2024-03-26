mod ingredient;

use glib::Object;
use gtk::glib::{
    self, subclass::types::ObjectSubclassIsExt,
};
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct IngredientObject(ObjectSubclass<ingredient::IngredientObject>);
}

impl IngredientObject {
    pub fn new(
        kind: String,
        name: String,
        amount: u32,
    ) -> Self {
        Object::builder()
            .property("kind", kind)
            .property("name", name)
            .property("amount", amount)
            .build()
    }
    pub fn data(&self) -> IngredientData {
        self.imp().data.borrow().clone()
    }
    pub fn from_data(data: IngredientData ) -> Self {
        Self::new(
            data.kind,
            data.name,
            data.amount
        )
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IngredientData {
    pub kind: String,
    pub name: String,
    pub amount: u32,
}

// impl<T: IngredientImpl> _ for IngredientData<T> {}
