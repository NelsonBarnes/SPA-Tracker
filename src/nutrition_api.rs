use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};
use thiserror::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct Nutrient {
    pub attr_id: u32,
    pub value: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiFood {
    pub food_name: String,
    pub serving_weight_grams: f32,
    pub full_nutrients: Vec<Nutrient>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NutritionxResponse {
    pub foods: Vec<ApiFood>,
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),
    #[error("No results found for the given query")]
    NotFound,
}

pub fn get_nutrient_name_map() -> HashMap<u32, &'static str> {
    let nutrient_map: HashMap<u32, &str> = [
        (301, "calcium_ca"),
        (205, "carbohydrates"),
        (601, "cholesterol"),
        (208, "energy"),
        (606, "fatty_acids_saturated"),
        (204, "total_lipid_fat"),
        (605, "fatty_acids_trans"),
        (303, "iron_fe"),    
        (291, "fiber_dietary"),
        (306, "potassium_k"),
        (307, "sodium_na"),
        (203, "protein"),
        (269, "sugars_total"),
        (539, "sugars_added"),
        (324, "vitamin_d"),
        (513, "alanine"),
        (221, "alcohol_ethyl"),
        (511, "arginine"),
        (207, "ash"),
        (514, "aspartic_acid"),
        (454, "betaine"),
        (262, "caffeine"),
        (639, "campesterol"),
        (322, "carotene_alpha"),
        (321, "carotene_beta"),
        (326, "vitamin_d3"),
        (421, "choline_total"),
        (334, "cryptoxanthin_beta"),
        (312, "copper_cu"),
        (507, "cystine"),
        (268, "energy_kj"),
        (325, "vitamin_d2"),
        (645, "fatty_acids_monounsaturated"),
        (646, "fatty_acids_polyunsaturated"),
        (693, "fatty_acids_transmonoenoic"),
        (695, "fatty_acids_transpolyenoic"),
        (313, "fluoride_f"),
        (417, "folate_total"),
        (431, "folic_acid"),
        (435, "folate_dfe"),
        (432, "folate_food"),
        (212, "fructose"),
        (287, "galactose"),
        (515, "glutamic_acid"),
        (211, "glucose_dextrose"),
        (516, "glycine"),
        (512, "histidine"),
        (521, "hydroxyproline"),
        (503, "isoleucine"),
        (213, "lactose"),
        (504, "leucine"),
        (338, "lutein_zeaxanthin"),
        (337, "lycopene"),
        (505, "lysine"),
        (214, "maltose"),
        (506, "methionine"),
        (304, "magnesium_mg"),
        (428, "menaquinone"),
        (315, "manganese_mn"),
        (406, "niacin"),
        (573, "vitamin_e_added"),
        (578, "vitamin_b_added"),
        (257, "adjusted_protein"),
        (305, "phosphorus_p"),
        (410, "pantothenic_acid"),
        (508, "phenylalanine"),
        (636, "phytosterols"),
        (517, "proline"),
        (319, "retinol"),
        (405, "riboflavin"),
        (317, "selenium_se"),
        (518, "serine"),
        (641, "betasitosterol"),
        (209, "starch"),
        (638, "stigmasterol"),
        (210, "sucrose"),
        (263, "theobromine"),
        (404, "thiamin"),
        (502, "threonine"),
        (323, "vitamin_e_alphatocopherol"),
        (341, "tocopherol_beta"),
        (343, "tocopherol_delta"),
        (342, "tocopherol_gamma"),
        (501, "tryptophan"),
        (509, "tyrosine"),
        (510, "valine"),
        (318, "vitamin_a_iu"),
        (320, "vitamin_a_rae"),
        (418, "vitamin_b12"),
        (415, "vitamin_b6"),
        (401, "vitamin_c_total_ascorbic_acid"),
        (430, "vitamin_k_phylloquinone"),
        (429, "dihydrophylloquinone"),
        (255, "water"),
        (309, "zinc_zn"),
        (344, "tocotrienol_alpha"),
        (345, "tocotrienol_beta"),
        (346, "tocotrienol_gamma"),
        (347, "tocotrienol_delta")
    ]
    .iter()
    .cloned()
    .collect();

    nutrient_map
}

pub async fn query_nutritionx(query: String) -> Result<Vec<ApiFood>, QueryError> {
    let app_id = "68bfdaab";
    let app_key = "ad27f9e6dc09294cdebb0cc44b30433b";
    let client = Client::new();
    let url = "https://trackapi.nutritionix.com/v2/natural/nutrients";
    let nutrient_name_map = get_nutrient_name_map();
    let mut foods = Vec::new();

    let response = client
        .post(url)
        .header("x-app-id", app_id)
        .header("x-app-key", app_key)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?;

    if response.status().is_success() {
        let nutritionix_response: NutritionxResponse = response.json().await?;
        foods = nutritionix_response.foods;
    } else {
        return Err(QueryError::NotFound);
    }

    Ok(foods)
}

pub async fn query_nutritionx_cli() -> Result<Vec<NutritionxResponse>, Box<dyn Error>> {
    let app_id = "68bfdaab";
    let app_key = "ad27f9e6dc09294cdebb0cc44b30433b";
    let client = Client::new();
    let url = "https://trackapi.nutritionix.com/v2/natural/nutrients";
    let nutrient_name_map = get_nutrient_name_map();
    let mut collected_data = Vec::new();

    loop {
        print!("Enter a food item (or type 'exit' to quit): ");
        io::stdout().flush()?;
        let mut query = String::new();
        io::stdin().read_line(&mut query)?;
        let query = query.trim().to_string();

        if query.to_lowercase() == "exit" {
            break;
        }

        println!("{}", query);

        let response = client
            .post(url)
            .header("x-app-id", app_id)
            .header("x-app-key", app_key)
            .json(&serde_json::json!({ "query": query }))
            .send()
            .await?;

        if response.status().is_success() {
            let nutritionix_response: NutritionxResponse = response.json().await?;
            collected_data.push(nutritionix_response.clone());

            for food in nutritionix_response.foods {
                println!("Food Name: {}", food.food_name);
                println!("Serving Weight: {} grams", food.serving_weight_grams);
                println!("Full Nutrients:");
                for nutrient in &food.full_nutrients {
                    if let Some(nutrient_name) = nutrient_name_map.get(&nutrient.attr_id) {
                        println!(
                            "  Name: {}, Value: {} {}",
                            nutrient_name,
                            nutrient.value,
                            if *nutrient_name == "Energy" { "kcal" } else { "g" }
                        );
                    }
                }
                println!();
            }
        } else {
            println!("Food not found in Nutritionix database.");
        }
    }

    Ok(collected_data)
}