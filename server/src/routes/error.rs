use rocket::{
    catch,
    serde::{json::Json, Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorInfo<'r> {
    pub code: i64,
    pub message: &'r str,
}

#[catch(404)]
pub fn not_found<'r>() -> Json<ErrorInfo<'r>> {
    return Json(ErrorInfo {
        code: 404,
        message: "Route not found!",
    });
}
