mod recipe;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, Object},
};
use serde::{Deserialize, Serialize};

// use crate::food::{Food, FoodObject};
use crate::{
    ingredient::{IngredientData, IngredientObject},
    // recipe::RecipeObject
};

glib::wrapper! {
    pub struct Recipe(ObjectSubclass<recipe::Recipe>);
}

impl Recipe {
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
    ) -> RecipeData {
        let name = self.imp().name.borrow().clone();
        let ingredientlist = self
            .ingredientlist()
            .iter::<IngredientObject>()
            .filter_map(Result::ok)
            .map(|food_object| food_object.data())
            .collect();
        RecipeData { name, ingredientlist }
    }
    pub fn from_collection_data(
        data: RecipeData,
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
pub struct RecipeData {
    pub name: String,
    pub ingredientlist: Vec<IngredientData>,
}
