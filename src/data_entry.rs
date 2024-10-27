use crate::nutrition_api::{query_nutritionx_cli, get_nutrient_name_map, ApiFood, NutritionxResponse};

use rusqlite::{params, Connection, Result, ToSql, Error as RusqliteError};
use chrono::prelude::*;
use std::io::{self, Write};
use std::error::Error;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Food {
    pub name: String,
    pub weight_grams: f32,
    pub nutrients: HashMap<String, f32>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub foods: Vec<Food>,
    pub weights: Vec<f32>,
}

const KEYS: [&str; 99] = [
    "calcium_ca", "carbohydrates", "cholesterol", "energy", "fatty_acids_saturated",
    "total_lipid_fat", "fatty_acids_trans", "iron_fe", "fiber_dietary", "potassium_k",
    "sodium_na", "protein", "sugars_total", "sugars_added", "vitamin_d", "alanine",
    "alcohol_ethyl", "arginine", "ash", "aspartic_acid", "betaine", "caffeine",
    "campesterol", "carotene_alpha", "carotene_beta", "vitamin_d3",
    "choline_total", "cryptoxanthin_beta", "copper_cu", "cystine", "energy_kj",
    "vitamin_d2", "fatty_acids_monounsaturated", "fatty_acids_polyunsaturated",
    "fatty_acids_transmonoenoic", "fatty_acids_transpolyenoic", "fluoride_f", "folate_total",
    "folic_acid", "folate_dfe", "folate_food", "fructose", "galactose", "glutamic_acid",
    "glucose_dextrose", "glycine", "histidine", "hydroxyproline", "isoleucine", "lactose",
    "leucine", "lutein_zeaxanthin", "lycopene", "lysine", "maltose", "methionine",
    "magnesium_mg", "menaquinone", "manganese_mn", "niacin", "vitamin_e_added", "vitamin_b_added",
    "adjusted_protein", "phosphorus_p", "pantothenic_acid", "phenylalanine", "phytosterols",
    "proline", "retinol", "riboflavin", "selenium_se", "serine", "betasitosterol", "starch",
    "stigmasterol", "sucrose", "theobromine", "thiamin", "threonine", "vitamin_e_alphatocopherol",
    "tocopherol_beta", "tocopherol_delta", "tocopherol_gamma", "tryptophan", "tyrosine", "valine",
    "vitamin_a_iu", "vitamin_a_rae", "vitamin_b12", "vitamin_b6", "vitamin_c_total_ascorbic_acid",
    "vitamin_k_phylloquinone", "dihydrophylloquinone", "water", "zinc_zn",
    "tocotrienol_alpha", "tocotrienol_beta", "tocotrienol_gamma", "tocotrienol_delta"
];

const SQL_INSERT: &str = "INSERT INTO food_items (name, weight_grams, calcium_ca, carbohydrates, 
cholesterol, energy, fatty_acids_saturated, total_lipid_fat, fatty_acids_trans, iron_fe,
fiber_dietary, potassium_k, sodium_na, protein, sugars_total, sugars_added, vitamin_d, alanine,
alcohol_ethyl, arginine, ash, aspartic_acid, betaine, caffeine, campesterol, carotene_alpha, 
carotene_beta, vitamin_d3, choline_total, cryptoxanthin_beta, copper_cu, cystine, 
energy_kj, vitamin_d2, fatty_acids_monounsaturated, fatty_acids_polyunsaturated, 
fatty_acids_transmonoenoic, fatty_acids_transpolyenoic, fluoride_f, folate_total, folic_acid, 
folate_dfe, folate_food, fructose, galactose, glutamic_acid, glucose_dextrose, glycine, 
histidine, hydroxyproline, isoleucine, lactose, leucine, lutein_zeaxanthin, lycopene, lysine, 
maltose, methionine, magnesium_mg, menaquinone, manganese_mn, niacin, vitamin_e_added, 
vitamin_b_added, adjusted_protein, phosphorus_p, pantothenic_acid, phenylalanine, 
phytosterols, proline, retinol, riboflavin, selenium_se, serine, betasitosterol, starch, 
stigmasterol, sucrose, theobromine, thiamin, threonine, vitamin_e_alphatocopherol, 
tocopherol_beta, tocopherol_delta, tocopherol_gamma, tryptophan, tyrosine, valine, 
vitamin_a_iu, vitamin_a_rae, vitamin_b12, vitamin_b6, vitamin_c_total_ascorbic_acid, 
vitamin_k_phylloquinone, dihydrophylloquinone, water, zinc_zn, tocotrienol_alpha, 
tocotrienol_beta, tocotrienol_gamma, tocotrienol_delta) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 
?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, 
?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40, ?41, ?42, ?43, ?44, ?45, ?46, 
?47, ?48, ?49, ?50, ?51, ?52, ?53, ?54, ?55, ?56, ?57, ?58, ?59, ?60, ?61, ?62, ?63, ?64, ?65, 
?66, ?67, ?68, ?69, ?70, ?71, ?72, ?73, ?74, ?75, ?76, ?77, ?78, ?79, ?80, ?81, ?82, ?83, ?84, 
?85, ?86, ?87, ?88, ?89, ?90, ?91, ?92, ?93, ?94, ?95, ?96, ?97, ?98, ?99, ?100, ?101)";

pub fn get_foods() -> Result<Vec<Food>> {
    let conn = Connection::open("my_database.db")?;
    let mut foods = Vec::new();

    {
        let mut stmt = conn.prepare("
            SELECT name, weight_grams, calcium_ca, carbohydrates, 
                cholesterol, energy, fatty_acids_saturated, total_lipid_fat, fatty_acids_trans, iron_fe,
                fiber_dietary, potassium_k, sodium_na, protein, sugars_total, sugars_added, vitamin_d, alanine,
                alcohol_ethyl, arginine, ash, aspartic_acid, betaine, caffeine, campesterol, carotene_alpha, 
                carotene_beta, vitamin_d3, choline_total, cryptoxanthin_beta, copper_cu, cystine, 
                energy_kj, vitamin_d2, fatty_acids_monounsaturated, fatty_acids_polyunsaturated, 
                fatty_acids_transmonoenoic, fatty_acids_transpolyenoic, fluoride_f, folate_total, folic_acid, 
                folate_dfe, folate_food, fructose, galactose, glutamic_acid, glucose_dextrose, glycine, 
                histidine, hydroxyproline, isoleucine, lactose, leucine, lutein_zeaxanthin, lycopene, lysine, 
                maltose, methionine, magnesium_mg, menaquinone, manganese_mn, niacin, vitamin_e_added, 
                vitamin_b_added, adjusted_protein, phosphorus_p, pantothenic_acid, phenylalanine, 
                phytosterols, proline, retinol, riboflavin, selenium_se, serine, betasitosterol, starch, 
                stigmasterol, sucrose, theobromine, thiamin, threonine, vitamin_e_alphatocopherol, 
                tocopherol_beta, tocopherol_delta, tocopherol_gamma, tryptophan, tyrosine, valine, 
                vitamin_a_iu, vitamin_a_rae, vitamin_b12, vitamin_b6, vitamin_c_total_ascorbic_acid, 
                vitamin_k_phylloquinone, dihydrophylloquinone, water, zinc_zn, tocotrienol_alpha, 
                tocotrienol_beta, tocotrienol_gamma, tocotrienol_delta
            FROM food_items
        ")?;
    
        let food_iter = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let weight_grams: f32 = row.get(1)?;

            let mut nutrients = HashMap::new();

            let nutrient_names = [
                "calcium_ca", "carbohydrates", "cholesterol", "energy", 
                "fatty_acids_saturated", "total_lipid_fat", "fatty_acids_trans", 
                "iron_fe", "fiber_dietary", "potassium_k", "sodium_na", "protein", 
                "sugars_total", "sugars_added", "vitamin_d", "alanine", 
                "alcohol_ethyl", "arginine", "ash", "aspartic_acid", "betaine", 
                "caffeine", "campesterol", "carotene_alpha", "carotene_beta", 
                "vitamin_d3", "choline_total", "cryptoxanthin_beta", "copper_cu", 
                "cystine", "energy_kj", "vitamin_d2", "fatty_acids_monounsaturated", 
                "fatty_acids_polyunsaturated", "fatty_acids_transmonoenoic", 
                "fatty_acids_transpolyenoic", "fluoride_f", "folate_total", 
                "folic_acid", "folate_dfe", "folate_food", "fructose", "galactose", 
                "glutamic_acid", "glucose_dextrose", "glycine", "histidine", 
                "hydroxyproline", "isoleucine", "lactose", "leucine", 
                "lutein_zeaxanthin", "lycopene", "lysine", "maltose", "methionine", 
                "magnesium_mg", "menaquinone", "manganese_mn", "niacin", 
                "vitamin_e_added", "vitamin_b_added", "adjusted_protein", 
                "phosphorus_p", "pantothenic_acid", "phenylalanine", "phytosterols", 
                "proline", "retinol", "riboflavin", "selenium_se", "serine", 
                "betasitosterol", "starch", "stigmasterol", "sucrose", 
                "theobromine", "thiamin", "threonine", "vitamin_e_alphatocopherol", 
                "tocopherol_beta", "tocopherol_delta", "tocopherol_gamma", 
                "tryptophan", "tyrosine", "valine", "vitamin_a_iu", 
                "vitamin_a_rae", "vitamin_b12", "vitamin_b6", 
                "vitamin_c_total_ascorbic_acid", "vitamin_k_phylloquinone", 
                "dihydrophylloquinone", "water", "zinc_zn", "tocotrienol_alpha", 
                "tocotrienol_beta", "tocotrienol_gamma", "tocotrienol_delta"
            ];

            for (i, &nutrient_name) in nutrient_names.iter().enumerate() {
                let value: f32 = row.get(i + 2)?;
                nutrients.insert(nutrient_name.to_string(), value);
            }

            Ok(Food {
                name,
                weight_grams,
                nutrients,
            })
        })?;

        for food in food_iter {
            foods.push(food?);
        }
    }

    let _ = conn.close();

    Ok(foods)
}

pub fn add_food_items(foods: Vec<ApiFood>) -> Result<(), Box<dyn Error>> {
    let nutrient_name_map = get_nutrient_name_map();
    let conn = Connection::open("my_database.db")?;

    for food in foods {
        let mut nutrients: HashMap<&str, f32> = HashMap::new();

        for nutrient in &food.full_nutrients {
            if let Some(&nutrient_name) = nutrient_name_map.get(&nutrient.attr_id) {
                nutrients.insert(nutrient_name, nutrient.value);
            }
        }

        match insert_food_item(&conn, &food, &nutrients) {
            Ok(_) => {},
            Err(e) => return Err(Box::new(e))
        }
    }

    let _ = conn.close();

    Ok(())
}

fn insert_food_item(
    conn: &Connection,
    food: &ApiFood,
    nutrients: &HashMap<&str, f32>,
) -> Result<usize> {
    let params = create_params(food, nutrients);
    conn.execute(SQL_INSERT, params.as_slice())
}

fn create_params<'a>(food: &'a ApiFood, nutrients: &'a HashMap<&str, f32>) -> Vec<&'a dyn ToSql> {
    let mut params: Vec<&'a dyn ToSql> = Vec::new();
    params.push(&food.food_name);
    params.push(&food.serving_weight_grams);

    for &key in &KEYS {
        params.push(nutrients.get(key).unwrap_or(&0.0));
    }

    params
}

pub fn add_pantry_item() -> Result<(), Box<dyn Error>> {
    // Open or create the database file
    let conn = Connection::open("my_database.db")?;

    loop {
        let food_name = read_input("Enter food name (or type 'exit' to cancel): ");
        if food_name.to_lowercase() == "exit" {
            println!("Operation cancelled by the user.");
            return Ok(());
        }

        let food_id: Result<i32> = conn.query_row(
            "SELECT id FROM food_items WHERE name = ?1",
            params![food_name],
            |row| row.get(0),
        );

        let food_id = match food_id {
            Ok(id) => id,
            Err(_) => {
                println!("Food name not found. Please try again.");
                continue; // Prompt for another food name
            }
        };

        let weight_grams: f32 = match read_input("Weight grams: ").trim().parse() {
            Ok(weight) => weight,
            Err(_) => {
                println!("Invalid input for weight grams.");
                return Ok(()); // Exit the function without error
            }
        };

        conn.execute(
            "INSERT INTO pantry (food_id, weight_grams, weight_grams_remaining) VALUES (?1, ?2, ?2)",
            params![food_id, weight_grams],
        )?;

        println!("Pantry item added.");
        return Ok(());
    }
}

pub fn add_recipe() -> Result<(), Box<dyn Error>> {
    // Open or create the database file
    let conn = Connection::open("my_database.db")?;

    let recipe_name = read_input("Enter recipe name: ");

    // Check if the recipe name already exists
    let recipe_exists: Result<i32> = conn.query_row(
        "SELECT id FROM recipes WHERE name = ?1",
        params![recipe_name],
        |row| row.get(0),
    );

    if let Ok(_) = recipe_exists {
        println!("Recipe name already exists. Please enter a different name.");
        return Ok(());
    }

    conn.execute(
        "INSERT INTO recipes (name) VALUES (?1)",
        params![recipe_name],
    )?;

    let recipe_id: i32 = conn.query_row(
        "SELECT id FROM recipes WHERE name = ?1",
        params![recipe_name],
        |row| row.get(0),
    )?;

    loop {
        let food_name = read_input("Enter food name for ingredient (or 'done' to finish): ");
        if food_name.trim().to_lowercase() == "done" {
            break;
        }

        let food_id: Result<i32> = conn.query_row(
            "SELECT id FROM food_items WHERE name = ?1",
            params![food_name],
            |row| row.get(0),
        );

        match food_id {
            Ok(id) => {
                let weight_grams: f32 = loop {
                    let input = read_input("Enter weight in grams: ");
                    match input.trim().parse() {
                        Ok(value) => break value,
                        Err(_) => println!("Invalid input for weight grams. Please enter a valid number."),
                    }
                };

                conn.execute(
                    "INSERT INTO recipe_ingredients (recipe_id, food_id, weight_grams) VALUES (?1, ?2, ?3)",
                    params![recipe_id, id, weight_grams],
                )?;
                println!("Ingredient added.");
            }
            Err(_) => {
                println!("Food item not found. Please try again.");
            }
        }
    }

    println!("Recipe added.");
    Ok(())
}


pub fn add_entry() -> Result<(), Box<dyn Error>> {
    // Open or create the database file
    let conn = Connection::open("my_database.db")?;

    let now = Utc::now();

    // Format the timestamp as a string
    let timestamp = now.format("%m-%d-%y %H:%M:%S").to_string();

    conn.execute(
        "INSERT INTO entries (timestamp) VALUES (?1)",
        params![timestamp],
    )?;

    let entry_id: i32 = conn.query_row(
        "SELECT id FROM entries WHERE timestamp = ?1",
        params![timestamp],
        |row| row.get(0),
    )?;

    loop {
        let food_name = read_input("Enter food name for entry (or 'done' to finish): ");
        if food_name.trim().to_lowercase() == "done" {
            break;
        }

        let food_id_result: Result<i32> = conn.query_row(
            "SELECT id FROM food_items WHERE name = ?1",
            params![food_name],
            |row| row.get(0),
        );

        match food_id_result {
            Ok(food_id) => {
                let weight_grams: f32 = loop {
                    let weight_input = read_input("Enter weight in grams: ");
                    match weight_input.trim().parse() {
                        Ok(weight) => break weight,
                        Err(_) => println!("Please enter a valid number for weight."),
                    }
                };

                conn.execute(
                    "INSERT INTO entry_foods (entry_id, food_id, weight_grams) VALUES (?1, ?2, ?3)",
                    params![entry_id, food_id, weight_grams],
                )?;
                println!("Ingredient added.");
            }
            Err(_) => {
                println!("Food item not found. Please try again.");
            }
        }
    }

    println!("Entry added.");

    Ok(())
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn create_database() -> Result<(), Box<dyn Error>> {
    // Open or create the database file
    let conn = Connection::open("my_database.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS food_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            weight_grams REAL NOT NULL,
            calcium_ca REAL NOT NULL,
            carbohydrates REAL NOT NULL,
            cholesterol REAL NOT NULL,
            energy REAL NOT NULL,
            fatty_acids_saturated REAL NOT NULL,
            total_lipid_fat REAL NOT NULL,
            fatty_acids_trans REAL NOT NULL,
            iron_fe REAL NOT NULL,
            fiber_dietary REAL NOT NULL,
            potassium_k REAL NOT NULL,
            sodium_na REAL NOT NULL,
            protein REAL NOT NULL,
            sugars_total REAL NOT NULL,
            sugars_added REAL NOT NULL,
            vitamin_d REAL NOT NULL,
            alanine REAL NOT NULL,
            alcohol_ethyl REAL NOT NULL,
            arginine REAL NOT NULL,
            ash REAL NOT NULL,
            aspartic_acid REAL NOT NULL,
            betaine REAL NOT NULL,
            caffeine REAL NOT NULL,
            campesterol REAL NOT NULL,
            carotene_alpha REAL NOT NULL,
            carotene_beta REAL NOT NULL,
            vitamin_d3 REAL NOT NULL,
            choline_total REAL NOT NULL,
            cryptoxanthin_beta REAL NOT NULL,
            copper_cu REAL NOT NULL,
            cystine REAL NOT NULL,
            energy_kj REAL NOT NULL,
            vitamin_d2 REAL NOT NULL,
            fatty_acids_monounsaturated REAL NOT NULL,
            fatty_acids_polyunsaturated REAL NOT NULL,
            fatty_acids_transmonoenoic REAL NOT NULL,
            fatty_acids_transpolyenoic REAL NOT NULL,
            fluoride_f REAL NOT NULL,
            folate_total REAL NOT NULL,
            folic_acid REAL NOT NULL,
            folate_dfe REAL NOT NULL,
            folate_food REAL NOT NULL,
            fructose REAL NOT NULL,
            galactose REAL NOT NULL,
            glutamic_acid REAL NOT NULL,
            glucose_dextrose REAL NOT NULL,
            glycine REAL NOT NULL,
            histidine REAL NOT NULL,
            hydroxyproline REAL NOT NULL,
            isoleucine REAL NOT NULL,
            lactose REAL NOT NULL,
            leucine REAL NOT NULL,
            lutein_zeaxanthin REAL NOT NULL,
            lycopene REAL NOT NULL,
            lysine REAL NOT NULL,
            maltose REAL NOT NULL,
            methionine REAL NOT NULL,
            magnesium_mg REAL NOT NULL,
            menaquinone REAL NOT NULL,
            manganese_mn REAL NOT NULL,
            niacin REAL NOT NULL,
            vitamin_e_added REAL NOT NULL,
            vitamin_b_added REAL NOT NULL,
            adjusted_protein REAL NOT NULL,
            phosphorus_p REAL NOT NULL,
            pantothenic_acid REAL NOT NULL,
            phenylalanine REAL NOT NULL,
            phytosterols REAL NOT NULL,
            proline REAL NOT NULL,
            retinol REAL NOT NULL,
            riboflavin REAL NOT NULL,
            selenium_se REAL NOT NULL,
            serine REAL NOT NULL,
            betasitosterol REAL NOT NULL,
            starch REAL NOT NULL,
            stigmasterol REAL NOT NULL,
            sucrose REAL NOT NULL,
            theobromine REAL NOT NULL,
            thiamin REAL NOT NULL,
            threonine REAL NOT NULL,
            vitamin_e_alphatocopherol REAL NOT NULL,
            tocopherol_beta REAL NOT NULL,
            tocopherol_delta REAL NOT NULL,
            tocopherol_gamma REAL NOT NULL,
            tryptophan REAL NOT NULL,
            tyrosine REAL NOT NULL,
            valine REAL NOT NULL,
            vitamin_a_iu REAL NOT NULL,
            vitamin_a_rae REAL NOT NULL,
            vitamin_b12 REAL NOT NULL,
            vitamin_b6 REAL NOT NULL,
            vitamin_c_total_ascorbic_acid REAL NOT NULL,
            vitamin_k_phylloquinone REAL NOT NULL,
            dihydrophylloquinone REAL NOT NULL,
            water REAL NOT NULL,
            zinc_zn REAL NOT NULL,
            tocotrienol_alpha REAL NOT NULL,
            tocotrienol_beta REAL NOT NULL,
            tocotrienol_gamma REAL NOT NULL,
            tocotrienol_delta REAL NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pantry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            food_id INTEGER NOT NULL,
            weight_grams REAL NOT NULL,
            weight_grams_remaining REAL NOT NULL,
            FOREIGN KEY (food_id) REFERENCES food_items (id)
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recipe_ingredients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            food_id INTEGER NOT NULL,
            weight_grams REAL NOT NULL,
            FOREIGN KEY (recipe_id) REFERENCES recipes (id),
            FOREIGN KEY (food_id) REFERENCES food_items (id),
            UNIQUE (recipe_id, food_id)
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entry_foods (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            entry_id INTEGER NOT NULL,
            food_id INTEGER NOT NULL,
            weight_grams REAL NOT NULL,
            FOREIGN KEY (entry_id) REFERENCES entries (id),
            FOREIGN KEY (food_id) REFERENCES food_items (id),
            UNIQUE (entry_id, food_id)
        )",
        [],
    )?;

    let _ = conn.close();

    Ok(())
}
