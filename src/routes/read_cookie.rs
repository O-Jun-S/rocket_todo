use rocket::http::{Cookie, CookieJar};


#[rocket::get("/c")]
pub async fn read_cookie(jar: &CookieJar<'_>) -> String {
    jar.add(Cookie::new("name", "val"));
    "I gave you a cookie!".to_string()
}
