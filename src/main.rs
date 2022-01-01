extern crate rocket;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[rocket::main]
async fn main() {
    let app = rocket::build()
            .mount("/", rocket::routes![index])
            .launch()
            .await;
    
    match app {
        Ok(()) => println!("Rocket launched successfully."),
        _ => println!("Some errors occured...")
    }
}
