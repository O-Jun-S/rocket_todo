use rocket::serde::{Serialize, Deserialize};
use rocket::form::{FromForm};

#[derive(FromForm)]
pub struct TodoDesc<'r> {
    pub description: &'r str,
}


#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub description: String,
    pub completed: bool,
}
