use crate::user::auth::APIToken;
use rocket_contrib::json::JsonValue;

#[get("/test")]
pub fn test(token: APIToken) -> String {
    format!("hello {}", token.0)
}

#[get("/test", rank = 2)]
pub fn test_error() -> JsonValue {
    json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    )
}