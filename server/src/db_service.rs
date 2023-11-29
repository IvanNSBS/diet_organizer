use rusqlite::Connection;
use std::{fs, path::Path, io::Error, io::ErrorKind, result::Result};

use crate::db_food::{Food, Recipe, FoodAndAmount};

pub trait DBAdapter {
    fn start(&mut self);
    fn get_all_foods(&mut self) -> Result<Vec<Food>, ()>;
    fn get_food_by_id(&mut self, id: i32) -> Result<Food, ()>;
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

                // match self.get_all_foods() {
                //     Ok(x) => println!("all foods: {:?}", x),
                //     Err(_) => println!("Error while getting foods")
                // }

                // match self.get_recipe_by_id(0) {
                //     Ok(x) => println!("recipe with id 0: {:?}", x),
                //     Err(_) => println!("error while getting recipe")
                // }
            },
            Err(error) => println!("Error while reading sql scripts: {}", error)
        }
    }

    fn get_all_foods(&mut self) -> Result<Vec<Food>, ()> {
        let mut statement = self.conn.prepare("
            SELECT 
                id, 
                uuid, 
                name, 
                protein, 
                carbs, 
                fat, 
                fiber, 
                unit_type 
            FROM 
                food").unwrap();
        let foods_iter = statement.query_map([], |row| {
            
            Ok(Food {
                id: row.get(0).unwrap(),
                uuid: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                protein: row.get(3).unwrap(),
                carbs: row.get(4).unwrap(),
                fat: row.get(5).unwrap(),
                fiber: row.get(6).unwrap(),
                unit_type: row.get(7).unwrap()
            })
        }).expect("Invalid query");

        let all_foods: Vec<Food> = foods_iter.map(|x| x.unwrap()).collect();
        return Ok(all_foods);
    }

    fn get_food_by_id(&mut self, id: i32) -> Result<Food, ()> {
        let mut recipes_stmt = self.conn.prepare(
            "SELECT   
                    food.id,
                    food.uuid,
                    food.name,
                    food.carbs,
                    food.protein,
                    food.fat,
                    food.fiber,
                    food.unit_type
                FROM 
                    food
                WHERE 
                    food.id=?1
                "
            ).expect("Error while preparing sql statement");

        let mut rows = recipes_stmt.query([id]).expect("error with sqlite query");
        while let Some(row) = rows.next().expect("") {

            // let unit_type_char: u8 = row.get(7).unwrap();
            let food = Food {
                id: row.get(0).unwrap(),
                uuid: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                carbs: row.get(3).unwrap(),
                protein: row.get(4).unwrap(),
                fat: row.get(5).unwrap(),
                fiber: row.get(6).unwrap(),
                unit_type: row.get(7).unwrap()
            };

            return Ok(food);
        }

        return Err(());
    }

    fn get_all_recipes(&mut self) -> Result<Vec<Recipe>, ()> {
        let mut recipe_ids: Vec<i32> = Vec::new();
        let mut recipes: Vec<Recipe> = Vec::new();

        {
            let mut recipes_stmt = self.conn.prepare(
                "SELECT 
                        recipe.id, 
                    FROM 
                        recipe"
                ).expect("Error while preparing sql statement");
    
            let mut rows = recipes_stmt.query([]).expect("error with sqlite query");
    
            while let Some(row) = rows.next().expect("") {
                let recipe_id: i32 = row.get(0).unwrap(); 
                recipe_ids.push(recipe_id);
            }
        }

        for id in recipe_ids {
            let recipe = self.get_recipe_by_id(id);
            match recipe {
                Ok(recipe) => recipes.push(recipe),
                _ => ()
            }
        }

        return Ok(recipes);
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
                food.fat,
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
        let mut recipe_foods: Vec<FoodAndAmount> = Vec::new();
        
        while let Some(row) = rows.next().expect("") {
            recipe_id = row.get(0).unwrap();
            recipe_uuid = row.get(1).unwrap();
            recipe_name = row.get(2).unwrap();
            
            
            let food = Food {
                id: row.get(3).unwrap(),
                uuid: row.get(4).unwrap(),
                name: row.get(5).unwrap(),
                carbs: row.get(6).unwrap(),
                protein: row.get(7).unwrap(),
                fat: row.get(8).unwrap(),
                fiber: row.get(9).unwrap(),
                unit_type: row.get(10).unwrap()
            };

            let food_amount = FoodAndAmount {
                food: food,
                amount: row.get(10).unwrap()
            };

            recipe_foods.push(food_amount);
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