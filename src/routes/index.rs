use rocket_dyn_templates::Template;


#[derive(serde::Serialize)]
struct Empty {
}


#[rocket::get("/")]
pub async fn index() -> Template {
    let context = Empty{};
    Template::render("index", &context)
}
