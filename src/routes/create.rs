use crate::routes::models;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TodoVec {
    todos: Vec<models::Todo>,
}


impl TodoVec {
    fn push(&mut self, todo: models::Todo) {
        self.todos.push(todo);
    }
}


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

    // 元のJSONを受け取る
    let json_result: Result<TodoVec, serde_json::Error> = match jar.get("body") {
        Some(body) => serde_json::from_str(&body.value().to_string()),
        None => serde_json::from_str(""),
    };

    let mut todos: TodoVec = match json_result {
        Ok(body) => body,
        Err(e) => return format!("Error at mut todos:  {}", e),
    };

    let todo = models::Todo{ description: desc, completed: false };
    todos.push(todo);

    let todos_result = serde_json::to_string(&todos);
    let todos_string = match todos_result {
        Ok(todos_str) => todos_str,
        Err(e) => return format!("Error at todos_string: {}", e),
    };
    jar.add(Cookie::new("body", todos_string));

    format!("You have the cookie: {:?}, eh??", todos)
}
