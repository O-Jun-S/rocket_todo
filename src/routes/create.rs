use crate::routes::models;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};


// number: the number of todo tasks
#[rocket::post("/create", data = "<todo_desc>")]
pub async fn create_todo(
    jar: &CookieJar<'_>,
    todo_desc: Option<Form<models::TodoDesc<'_>>>,
) -> String {
    let desc: String = match todo_desc {
        Some(d) => d.into_inner().description.to_string(),
        None => "".to_string(),
    };

    let todo = models::Todo{ description: desc, completed: false };
    jar.add(Cookie::new("body", serde_json::to_string(&todo).unwrap()));

    serde_json::to_string(&todo).unwrap()
}
