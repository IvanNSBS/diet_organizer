use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Food {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
    pub fiber: f32,
    pub unit_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoodAndAmount {
    pub food: Food,
    pub amount: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe { 
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub foods: Vec<FoodAndAmount>
}

// #[derive(Serialize, Deserialize)]
// struct Meal {
//     id: String,
//     foods: Vec<String>,
//     recipes: [String; MAX_FOODS_PER_MEAL]
// }

// #[derive(Serialize, Deserialize)]
// struct EatingDay { 
//     id: String,
//     name: String,
//     meals: [String; MEALS_PER_DAY]    
// }