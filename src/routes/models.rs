use rocket::serde::{Serialize, Deserialize};
use rocket::form::{FromForm};

#[derive(FromForm)]
pub struct TodoDesc<'r> {
    #[field(name = "todo_desc")]
    pub description: &'r str,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub description: String,
    pub completed: bool,
}
