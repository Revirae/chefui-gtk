mod recipe_collection;

// use adw::prelude::*;
// use adw::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, subclass::types::ObjectSubclassIsExt, Object}, prelude::ListModelExtManual,
};
use serde::{Deserialize, Serialize};

use crate::ingredient::{IngredientData, IngredientObject};

glib::wrapper! {
    pub struct RecipeCollection(ObjectSubclass<recipe_collection::RecipeCollection>);
}

impl RecipeCollection {
    pub fn new(
        name: &str,
        ingredientlist: gio::ListStore,
    ) -> Self {
        Object::builder()
            .property("name", name)
            .property("ingredientlist", ingredientlist)
            .build()
    }
    pub fn to_collection_data(
        &self,
    ) -> RecipeCollectionData {
        let name = self.imp().name.borrow().clone();
        let ingredientlist = self
            .ingredientlist()
            .iter::<IngredientObject>()
            .filter_map(Result::ok)
            .map(|ingredient_object| ingredient_object.data())
            .collect();
        RecipeCollectionData { name, ingredientlist }
    }
    pub fn from_collection_data(
        data: RecipeCollectionData,
    ) -> Self {
        let name = data.name;
        let ingredient_to_extend: Vec<IngredientObject> = data
            .ingredientlist
            .into_iter()
            .map(IngredientObject::from_data)
            .collect();
        let ingredientlist =
            gio::ListStore::new::<IngredientObject>();
        ingredientlist.extend_from_slice(&ingredient_to_extend);

        Self::new(&name, ingredientlist)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RecipeCollectionData {
    pub name: String,
    pub ingredientlist: Vec<IngredientData>,
}
