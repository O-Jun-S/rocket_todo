extern crate rocket;
extern crate serde;
extern crate rocket_dyn_templates;

mod routes;

use rocket_dyn_templates::Template;

#[derive(serde::Serialize)]
struct Empty {
}


#[rocket::main]
async fn main() {
    let app = rocket::build()
            .mount("/", rocket::routes![routes::index::index, routes::create::create_todo])
            .attach(Template::fairing())
            .launch()
            .await;
    
    match app {
        Ok(()) => println!("Rocket launched successfully."),
        _ => println!("Some errors occured...")
    }
}
