use rusqlite::Connection;
use std::{fs, path::Path, io::Error, io::ErrorKind, result::Result};

use crate::db_food::{Food, Recipe};

pub trait DBAdapter {
    fn start(&mut self);
    fn get_all_foods(&mut self) -> Result<Vec<Food>, ()>;
    fn get_food_by_id(&mut self) -> Result<Food, ()>;
    fn get_all_recipes(&mut self) -> Result<Vec<Recipe>, ()>;
    fn get_recipe_by_id(&mut self, id: i32) -> Result<Recipe, ()>;
}

pub struct SqliteAdapter { 
    conn: Connection
}

impl SqliteAdapter {
    pub fn new() -> SqliteAdapter {
        let conn = Connection::open_in_memory().unwrap();
        SqliteAdapter { conn: conn }
    }
}

impl DBAdapter for SqliteAdapter {
    fn start(&mut self) {
        let file = read_file_text("sql_scripts\\init_db.sql");
        match file {
            Ok(content) => {
                let r = self.conn.execute_batch(&content);
                match r {
                    Ok(_) => println!("Script executed!"),
                    Err(err) => println!("Error while executing script: {}", err.to_string())
                }

                match self.get_all_foods() {
                    Ok(x) => println!("all foods: {:?}", x),
                    Err(_) => println!("Error while getting foods")
                }

                match self.get_recipe_by_id(0) {
                    Ok(x) => println!("recipe with id 0: {:?}", x),
                    Err(_) => println!("error while getting recipe")
                }
            },
            Err(error) => println!("Error while reading sql scripts: {}", error)
        }
    }

    fn get_all_foods(&mut self) -> Result<Vec<Food>, ()> {
        let mut statement = self.conn.prepare("SELECT id, uuid, name, protein, carbs, fat FROM food").unwrap();
        let foods_iter = statement.query_map([], |row| {
            Ok(Food {
                id: row.get(0).unwrap(),
                uuid: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                protein: row.get(3).unwrap(),
                carbs: row.get(4).unwrap(),
                fat: row.get(5).unwrap(),
            })
        }).expect("Invalid query");

        let all_foods: Vec<Food> = foods_iter.map(|x| x.unwrap()).collect();
        return Ok(all_foods);
    }

    fn get_food_by_id(&mut self) -> Result<Food, ()> {
        todo!()
    }

    fn get_all_recipes(&mut self) -> Result<Vec<Recipe>, ()> {
        todo!();
    }

    fn get_recipe_by_id(&mut self, id: i32) -> Result<Recipe, ()> {
        let mut statement = self.conn.prepare(
        "SELECT 
                recipe.id as r_id, 
                recipe.uuid as r_uuid, 
                recipe.name as r_name, 
                food.id, 
                food.uuid, 
                food.name, 
                food.carbs, 
                food.protein, 
                food.fiber, 
                food.unit_type, 
                rel_recipe_food.food_amount as amount
            FROM 
                recipe
            JOIN 
                food, rel_recipe_food ON food.id=rel_recipe_food.food_id AND 
                rel_recipe_food.recipe_id=recipe.id AND
                recipe.id= ?1;"
        ).expect("Error while preparing sql statement");

        let mut rows = statement.query([id]).expect("error with sqlite query");
        let mut recipe_id: i32 = -1;
        let mut recipe_uuid: String = String::from("");
        let mut recipe_name: String = String::from("");
        let mut recipe_foods: Vec<Food> = Vec::new();

        while let Some(row) = rows.next().expect("") {
            recipe_id = row.get(0).unwrap();
            recipe_uuid = row.get(1).unwrap();
            recipe_name = row.get(2).unwrap();

            let food = Food {
                id: row.get(3).unwrap(),
                uuid: row.get(4).unwrap(),
                name: row.get(5).unwrap(),
                protein: row.get(6).unwrap(),
                carbs: row.get(7).unwrap(),
                fat: row.get(8).unwrap(),
            };
            recipe_foods.push(food);
        }

        let recipe = Recipe {
            id: recipe_id,
            uuid: recipe_uuid,
            name: recipe_name,
            foods: recipe_foods
        };

        return Ok(recipe);
    }
}

fn read_file_text(path: &str) -> Result<String, Error> {
    if !Path::new(path).exists() {
        return Err(Error::new(ErrorKind::NotFound, "File Not Found"));
    }

    let file_content = fs::read_to_string(path);
    return file_content;
}