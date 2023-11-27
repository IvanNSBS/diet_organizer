use serde::{Deserialize, Serialize};
use serde_json::Result;

const MEALS_PER_DAY: usize = 12;
const MAX_FOODS_PER_RECIPE:  usize = 20;
const MAX_FOODS_PER_MEAL:  usize = 10;

#[derive(Serialize, Deserialize)]
struct Food {
    id: String,
    name: String,
    protein: i32,    
    carbs: i32,    
    fat: i32,    
}

#[derive(Serialize, Deserialize)]
struct Recipe { 
    id: String,
    foods: [String; MAX_FOODS_PER_RECIPE]
}

#[derive(Serialize, Deserialize)]
struct Meal {
    id: String,
    foods: Vec<String>,
    recipes: [String; MAX_FOODS_PER_MEAL]
}

#[derive(Serialize, Deserialize)]
struct EatingDay { 
    id: String,
    name: String,
    meals: [String; MEALS_PER_DAY]    
}