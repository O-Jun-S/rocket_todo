use rocket::form::{FromForm};

#[derive(FromForm)]
pub struct Todo<'r> {
    #[field(validate = len(1..))]
    pub description: &'r str,

    #[field(name = "done")]
    completed: bool,
}
