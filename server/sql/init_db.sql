CREATE TABLE food (
    id VARCHAR(128) PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    carbs INTEGER,
    protein INTEGER,
    fat INTEGER,
    measure_unit_type INTEGER,
    measure_unit_amount INTEGER
);

INSERT INTO food (id, name, carbs, protein, fat, measure_unit_type, measure_unit_amount)
VALUES ('12312', 'test_food', 20, 30, 40, 0, 100);

-- CREATE TABLE recipe (
--     id    TEXT PRIMARY KEY,
--     foods ,
-- );

-- CREATE TABLE eating_day (
--     id TEXT PRIMARY KEY,
-- );

-- #[derive(Serialize, Deserialize)]
-- struct Meal {
--     id: String,
--     foods: Vec<String>,
--     recipes: [String; MAX_FOODS_PER_MEAL]
-- }

-- #[derive(Serialize, Deserialize)]
-- struct EatingDay { 
--     id: String,
--     name: String,
--     meals: [String; MEALS_PER_DAY]    
-- }

-- INSERT INTO food (id, name, carbs, protein, fats, measure_unit_type, measure_unit_amount)
-- VALUES ();