mod food;

use glib::Object;
use gtk::glib::{
    self, subclass::types::ObjectSubclassIsExt,
};
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct FoodObject(ObjectSubclass<food::FoodObject>);
}

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
            // data.mustcreate,
            // data.mustupdate
        )
    }
}


pub enum Ingredient {
    FoodPortion(Food, usize),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Food {
    pub name: String,
    pub brand: String,
    pub cost: u32,
    pub weight: u32,
    pub volume: u32,
    // pub action: Action,
    // pub mustcreate: bool,
    // pub mustupdate: bool,
}

impl Food {
    pub fn amount(&self) -> u32 {
        if self.weight > 0 {
            self.weight
        } else {
            self.volume
        }
    }
    pub fn as_ingredient(
        &self,
        amount: usize,
    ) -> Ingredient {
        Ingredient::FoodPortion(self.clone(), amount)
    }
}

impl Ingredient {
    fn title(&self) -> String {
        match self {
            Ingredient::FoodPortion(food, _) => {
                food.name.clone()
            }
        }
    }
    fn subtitle(&self) -> String {
        match self {
            Ingredient::FoodPortion(food, _) => {
                food.brand.clone()
            }
        }
    }
    fn cost(&self) -> usize {
        match self {
            Ingredient::FoodPortion(food, amount) => {
                let ratio = (*amount as f32)
                    / (food.amount() as f32);
                ((food.cost as f32) * ratio) as usize
            }
        }
    }
}
