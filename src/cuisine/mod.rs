mod cuisine;
use rusqlite::Connection;

const DBFN: &'static str = "./chef.sqlite";

#[derive(Debug)]
pub struct Cuisine {
    
}

#[derive(Debug, Clone)]
pub struct FoodStore {
}

#[derive(Debug, Clone)]
pub struct IngredientStore {
    
}

impl Cuisine {
    fn link(
        &self,
    ) -> Result<Connection, rusqlite::Error> {
        Connection::open(DBFN)
    }
}
