CREATE TABLE food (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(128),
    name VARCHAR(128) NOT NULL,
    carbs REAL,
    protein REAL,
    fat REAL,
    fiber REAL,
    unit_type CHAR(1)
);

CREATE TABLE recipe (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(128),
    name VARCHAR(256) NOT NULL
);

CREATE TABLE rel_recipe_food (
    food_id INTEGER(128),
    recipe_id INTEGER(128),
    food_amount REAL,
    CONSTRAINT fk_food FOREIGN KEY (food_id) REFERENCES food(id),
    CONSTRAINT fk_recipe FOREIGN KEY (recipe_id) REFERENCES recipe(id),
    UNIQUE(food_id, recipe_id)
);

CREATE TABLE eating_day (
    day INTEGER PRIMARY KEY
);

CREATE TABLE meal (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid VARCHAR(128),
    meal_number INTEGER,
    eating_day INTEGER,
    CONSTRAINT fk_eating_day FOREIGN KEY(eating_day) REFERENCES eating_day(day)
);

CREATE TABLE rel_meal_food_recipes(
    meal_id VARCHAR(128),
    food_id VARCHAR(128),
    recipe_id VARCHAR(128),

    CONSTRAINT fk_meal FOREIGN KEY (meal_id) REFERENCES meal(id),
    CONSTRAINT fk_food FOREIGN KEY (food_id) REFERENCES food(id),
    CONSTRAINT fk_recipe FOREIGN KEY (recipe_id) REFERENCES recipe(id),
    UNIQUE(meal_id, food_id, recipe_id)
);

INSERT INTO food (id, uuid, carbs, protein, fat, fiber, unit_type, name) VALUES 
(0  ,'d7b936a5-2f87-459f-b4c5-7296211bc7c2', 0, 6, 5, 0, 'U', 'ovo'),
(1  ,'8a1f734d-c7ec-4220-aa0d-c11ce61fffc7', 0, 24, 3, 0, 'U', 'whey concentrado(dose)'),
(2  ,'385cf090-bb51-40b0-bab1-401d4646822f', 0, 20, 0, 0, 'G', 'frango(cru)'),
(3  ,'0de1713d-a8dd-4881-bb24-58a3adc10626', 0, 0, 95, 0, 'G', 'azeite'),
(4  ,'461fe27d-353c-4ae5-9761-18399a77406c', 0, 6, 5, 0, 'G', 'tapioca'),
(5  ,'e7d5b1cf-ec2b-4d26-b7ba-e77efc83f3b2', 28, 3, 0, 0.4, 'G', 'arroz'),
(6  ,'57fc4e86-4eec-455a-b1a2-9ebc95f3b50e', 74, 9.4, 0.4, 2.2, 'G', 'pao quadrado'),
(7  ,'9f516bcc-5d98-40e6-b9df-b297b995f0e3', 66.6, 13.9, 8.5, 9.1, 'G', 'aveia'),
(8  ,'b2943cc4-c5cf-4d27-94eb-5d1187f37482', 23, 13.9, 8.5, 2.6, 'G', 'banana'),
(9  ,'cef8856c-3d44-4717-99ad-e906e27c808b', 93, 0, 0, 0, 'G', 'dextrose'),
(10 ,'b5b8a9a1-80e7-4b81-8b40-4e3ab4cc6544', 14, 0.3, 0.2, 2.4, 'G', 'maça'),
(11 ,'c257665c-881d-4b1d-9b45-ffe1c10321b6', 20, 25, 50, 6, 'G', 'pasta de amendoim'),
(12 ,'95dc15ab-0768-4f34-92e5-466f2cdbaf5f', 8, 6, 1.5, 70, 'G', 'psyllium'),
(13 ,'9e800530-084e-46c4-92a4-c3a109a4783b', 0, 23, 8.7, 0, 'G', 'bisteca suina'),
(14 ,'012fe956-e046-419c-a09a-18f1a4697f0c', 0, 27, 14, 0, 'G', 'lombo suino'),
(15 ,'9a0b4e6a-4b87-4706-a186-547b4f156fcf', 22, 4, 2, 0.8, 'G', 'iogurte activia'),
(16 ,'789f5ccd-1a20-44db-9bb5-0ef9ab726992', 3, 12, 26, 0.0, 'G', 'requeijao'),
(17 ,'fd8cf32e-e20a-4d48-812d-86d1b5fe3717', 0, 21.3, 15.3, 0.0, 'G', 'charque(ponta de agulha)'),
(18 ,'81f579b1-1ba4-4c23-ae56-060a525cb454', 0.7, 24.5, 30.3, 0.0, 'G', 'queijo coalho'),
(19 ,'0391975f-43e9-46f0-b7b6-90487f77311c', 80, 7.2, 1.6, 2.8, 'G', 'cuscuz(cru)');

INSERT INTO recipe (id, uuid, name) VALUES
(0, '1a5e23a2-3281-49c3-b98d-357708ff4c7a', 'sanduiche natural');
INSERT INTO rel_recipe_food (recipe_id, food_id, food_amount) VALUES
(0, 0, 1),
(0, 6, 50),
(0, 16, 25),
(0, 17, 25);

INSERT INTO recipe (uuid, name) VALUES ('zxmcn,zxjkcyzx789iu', 'milkshake proteico');
INSERT INTO rel_recipe_food (recipe_id, food_id, food_amount) VALUES 
(1, 1, 1.5),
(1, 11, 30),
(1, 8, 150);