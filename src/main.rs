mod models;

extern crate rocket;
extern crate rocket_dyn_templates;
extern crate serde;

use rocket::form::Form;
use rocket_dyn_templates::Template;


#[derive(serde::Serialize)]
struct Empty {
}


#[rocket::get("/")]
fn index() -> Template {
    let context = test{};
    Template::render("index", &context)
}


// number: the number of todo tasks
#[rocket::post("/create", data = "<todo_desc>")]
async fn create_todo(todo_desc: Option<Form<models::TodoDesc<'_>>>) -> rocket::serde::json::Json<models::Todo> {
    let desc: String = match todo_desc {
        Some(d) => d.into_inner().description.to_string(),
        None => "".to_string(),
    };

    let todo = models::Todo{ description: desc, completed: false };

    rocket::serde::json::Json(todo)
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
