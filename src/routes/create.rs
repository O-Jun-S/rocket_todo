use crate::routes::models;

use rocket::form::Form;


// number: the number of todo tasks
#[rocket::post("/create", data = "<todo_desc>")]
pub async fn create_todo(todo_desc: Option<Form<models::TodoDesc<'_>>>) -> rocket::serde::json::Json<models::Todo> {
    let desc: String = match todo_desc {
        Some(d) => d.into_inner().description.to_string(),
        None => "".to_string(),
    };

    let todo = models::Todo{ description: desc, completed: false };

    rocket::serde::json::Json(todo)
}