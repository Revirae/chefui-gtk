#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
// use std::iter::Map;

use crate::collection::FoodCollectionData;
use crate::food::{Food, Ingredient};
use crate::action::Action;

impl super::Store {
    pub fn load_or_init(
        filename: String,
    ) -> rusqlite::Result<Self> {
        let store = super::Store {
            filename,
        };

        store.link()?.execute(
            "create table if not exists food (
                id integer primary key,
                name text not null unique,
                brand text,
                cost integer,
                weight integer,
                volume integer
            )",
            (),
        )?;

        Ok(store)
    }
    pub fn send_food(
        &self,
        collection: FoodCollectionData,
        commits: &Vec<Action>
    ) -> Result<(), rusqlite::Error> {
        let mut link = self.link()?;
        let tx = link.transaction()?;

        for action in commits.iter() {
            dbg!(action, collection.foodlist.clone());
            // dbg!(food);
            match action {
                Action::Create(food) => {
                    tx.execute(
                        "insert into food (
                        name, brand, cost, weight, volume
                        ) values (?1, ?2, ?3, ?4, ?5)",
                        rusqlite::params![
                            food.name,
                            food.brand,
                            food.cost,
                            food.weight,
                            food.volume
                        ],
                    )?;
                },
                Action::Update(name, food) => {
                        dbg!(name, food);
                        tx.execute(
                            "update food set 
                            name = ?1,
                            brand = ?2,
                            cost = ?3,
                            weight = ?4,
                            volume = ?5  
                             where name = ?6",
                            rusqlite::params![
                                food.name,
                                food.brand,
                                food.cost,
                                food.weight,
                                food.volume,
                                name
                            ]
                        )?;
                    }               
                _ => {
                    dbg!("Unresolved action evoqued");
                }
                
            }
        }
        tx.commit()?;
        Ok(())
    }
    pub fn get_food(
        &self,
    ) -> rusqlite::Result<FoodCollectionData> {
        let link = self.link()?;

        let mut stmt =
            link.prepare("SELECT * FROM food")?;

        let mut foodlist: Vec<Food> = Vec::new();

        stmt.query_and_then(
            (),
            |row| -> Result<Food, rusqlite::Error> {
                Ok(Food {
                    name: row.get(1)?,
                    brand: row.get(2)?,
                    cost: row.get(3)?,
                    weight: row.get(4)?,
                    volume: row.get(5)?,
                    ..Default::default()
                })
            },
        )?
        .for_each(|result| {
            if let Ok(food) = result {
                foodlist.push(food);
            }
        });

        Ok(FoodCollectionData {
            name: "all".to_owned(),
            foodlist,
        })
    }
}
