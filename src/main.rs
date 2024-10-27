use dioxus::prelude::*;

mod nutrition_api;
mod data_entry;

use crate::nutrition_api::{ApiFood, Nutrient, query_nutritionx, get_nutrient_name_map};
use crate::data_entry::{Food, create_database, add_food_items, get_foods};

fn main() {
    create_database();

    launch(|| {
        rsx! {
            style { {include_str!("../assets/style.css")} }
            Router::<Route> {}
        }
    });
}

// Turn off rustfmt since we're doing layouts and routes in the same enum
#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(HomeNavBar)]
        // The default route is always "/" unless otherwise specified
        #[route("/")]
        Home {},

        #[nest("/nutrition")]
        #[layout(NutritionNavBar)]
            // At "/blog", we want to show a list of blog posts
            #[route("/")]
            Nutrition {},

            #[route("/entry")]
            Entry {},

            #[route("/addfoods")]
            AddFoods {},

            #[route("/addpantryitem")]
            AddPantryItem {},

            #[route("/recipes")]
            AddRecipe {},

            #[route("/foods")]
            Foods {},

            

            // At "/blog/:name", we want to show a specific blog post, using the name slug
            #[route("/:name")]
            Recipe { name: String },
        #[end_layout]
        #[end_nest]

        #[nest("/excersize")]
        #[layout(ExcersizeNavBar)]
            // At "/blog", we want to show a list of blog posts
            #[route("/")]
            Excersize {},

            #[route("/aerobic")]
            Aerobic {},

            #[route("/anerobic")]
            Anerobic {},
        #[end_layout]
        #[end_nest]

    #[end_layout]

    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
fn HomeNavBar() -> Element {
    rsx! {
        div { class: "navbar",
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::Nutrition {}, "Nutrition" } }
                li { Link { to: Route::Excersize {}, "Excersize" } }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn NutritionNavBar() -> Element {
    rsx! {
        div { class: "navbar",
            ul {
                li { Link { to: Route::Entry{}, "Entry" } }
                li { Link { to: Route::AddFoods{}, "AddFoods" } }
                li { Link { to: Route::AddPantryItem {}, "AddPantryItem" } }
                li { Link { to: Route::AddRecipe {}, "Recipes" } }
                li { Link { to: Route::Foods{}, "Foods" } }
                
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn ExcersizeNavBar() -> Element {
    rsx! {
        div { class: "navbar",
            ul {
                li { Link { to: Route::Aerobic {}, "Aerobic" } }
                li { Link { to: Route::Anerobic {}, "Anerobic" } }
            }
            
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! { 
        body { class: "home",
            h1 { "Welcome to the Dioxus Blog!" }
        }
    }
}

#[component]
fn Nutrition() -> Element {
    rsx! {
    }
}

#[component]
fn Entry() -> Element {
    rsx! {
        h2 { "Foods" }
    }
}

#[component]
fn AddFoods() -> Element {
    let mut text = use_signal(|| String::new());

    let mut foods = use_signal(|| Vec::new());
    let foods_lock = foods.read();

    let nutrient_name_map = get_nutrient_name_map();

    rsx! {
        div {
            input {
                r#type: "text",
                value: "{text}",
                oninput: move |event| text.set(event.value()),
                onkeydown: move |event| {
                    if event.key() == Key::Enter {
                        let query_input = text.clone().to_string();
                        text.set(String::new());
                        
                        spawn(async move {
                            let resp = query_nutritionx(query_input).await;

                            match resp {
                                Ok(data) => { 
                                    foods.set(data.clone());
                                }
                                Err(err) => {}
                            }
                        });
                    }
                }
            }

            br {} br {}

            if foods.len() > 0 {
                button {
                    onclick: move |_| {
                        add_food_items(((*foods))().clone());
                        foods.set(Vec::new());
                    },
                    "Add to Database"
                }
            }

            for food in foods_lock.iter() {
                { let food: &ApiFood = food; }
                h3 { "{food.food_name} {food.serving_weight_grams}" }
                ul {
                    for nutrient in &food.full_nutrients {
                        if let Some(nutrient_name) = nutrient_name_map.get(&nutrient.attr_id) {
                            li { "{nutrient_name}: {nutrient.value}" }
                        }
                    }
                } 
            }
        }
    }
}

#[component]
fn AddRecipe() -> Element {
    let mut recipes: Vec<String> = Vec::new();

    // Add string slices to the vector
    recipes.push(String::from("Burger"));
    recipes.push(String::from("Fries"));
    recipes.push(String::from("Shake"));

    rsx! {

    }

    /*
    rsx! {  
        h2 { "Choose a recipe" }
        div { id: "blog-list",
            for r in &recipes {
                Link { to: Route::Recipe { name: r.to_string() },
                "{r.to_string()}"
                }
            }
        }
    }
    */
}

#[component]
fn AddPantryItem() -> Element {
    let mut foods = use_signal(|| Vec::new());
    let mut index = use_signal(|| 0);

    use_effect(move || {
        foods.set(get_foods().expect(""));
    });

    rsx! {
        select {
            onchange: move |event| { index.set(event.value().parse().expect("")); },
            { foods.iter().enumerate().map(|(i, food)| {
                rsx! { option { value: i as f64, "{food.name}" }, }
            }) }
        }

        br{}

        {
            foods.with(|foods| {
                if let Some(food) = foods.get(index()) {
                    format!("Selected food: {}", food.name)
                } else {
                    "No food selected".to_string()
                }
            })
        }

    }
}











#[component]
fn Foods() -> Element {
    let mut foods = use_signal(|| Vec::new());

    use_effect(move || {
        foods.set(get_foods().expect(""));
    });

    rsx! {
        for food in foods.iter() {
            p { "{food.name}" }
        }
    }
}

#[component]
fn Recipe(name: String) -> Element {
    rsx! {
        h2 { "{name}" }
    }
}

#[component]
fn Excersize() -> Element {
    rsx! {
        div {
            h2 { "Overview" }
        }
    }
}

#[component]
fn Aerobic() -> Element {
    rsx! {
        h2 { "Foods" }
    }
}

#[component]
fn Anerobic() -> Element {
    rsx! {
        h2 { "Foods" }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}