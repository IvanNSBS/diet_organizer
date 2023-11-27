use rusqlite::{Connection, Result, Params};
use std::{fs, path::{Path, PathBuf}, io::Error, io::ErrorKind, env};

use crate::db_food::Food;

pub trait DBAdapter {
    fn start(&mut self);
    fn get_registered_foods(&mut self);
    fn get_food_by_id(&mut self);
    fn get_foods_by_name(&mut self);
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
        let file = read_file_text("sql\\init_db.sql");
        match file {
            Ok(content) => {
                println!("File content:\n{}", content);
                let r = self.conn.execute_batch(&content);
                match r {
                    Ok(s) => println!("Script executed!"),
                    Err(err) => println!("Error while executing script: {}", err.to_string())
                }

                let mut statement = self.conn.prepare("SELECT name, protein, carbs, fat FROM food").unwrap();
                let foods_iter = statement.query_map([], |row| {
                    Ok(Food {
                        id: "asdad".to_string(),
                        name: row.get(0).unwrap(),
                        protein: row.get(1).unwrap(),
                        carbs: row.get(2).unwrap(),
                        fat: row.get(3).unwrap(),
                    })
                }).unwrap();

                println!("AUISDGASYUDASY");
                for food in foods_iter {
                    println!("Food: {:?}", food.unwrap());
                }
            },
            Err(error) => println!("Error while reading sql scripts: {}", error)
        }
    }

    fn get_registered_foods(&mut self) {
        todo!()
    }

    fn get_food_by_id(&mut self) {
        todo!()
    }

    fn get_foods_by_name(&mut self) {
        todo!()
    }
}

fn read_file_text(path: &str) -> Result<String, Error> {
    if !Path::new(path).exists() {
        return Err(Error::new(ErrorKind::NotFound, "File Not Found"));
    }

    let file_content = fs::read_to_string(path);
    return file_content;
}