mod models;

extern crate rocket;
extern crate rocket_dyn_templates;
extern crate serde;

use rocket::form::Form;
use rocket::http::{CookieJar, Cookie};
use rocket_dyn_templates::Template;


#[derive(serde::Serialize)]
struct test {
}


#[rocket::get("/")]
fn index() -> Template {
    let context = test{};
    Template::render("index", &context)
}


// number: the numbe of todo tasks
#[rocket::post("/create", data = "<todo>")]
async fn create_todo(todo: Option<Form<models::Todo<'_>>>) -> String {
    match todo {
        Some(todo) => format!("A todo: {} has just been created!", todo.description),
        None => "Failed to make todo.".to_string(),
    }
}


#[rocket::main]
async fn main() {
    let app = rocket::build()
            .mount("/", rocket::routes![index, create_todo])
            .attach(Template::fairing())
            .launch()
            .await;
    
    match app {
        Ok(()) => println!("Rocket launched successfully."),
        _ => println!("Some errors occured...")
    }
}
