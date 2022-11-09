use rocket::get;

#[get("/")]
pub async fn index() -> &'static str {
    return "Hello, world!";
}
