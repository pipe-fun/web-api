use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use jwt::{VerifyWithKey, SignWithKey};
use hmac::{Hmac, NewMac};
use std::collections::BTreeMap;
use sha2::Sha256;
use rocket_contrib::json::JsonValue;

pub struct APIToken(pub String);

#[get("/auth")]
pub fn authorized(token: APIToken) -> JsonValue {
    let name = token.0;
    json!( { "authorized": true, "user_name": name } )
}

#[get("/auth", rank = 2)]
pub fn not_authorized() -> JsonValue {
    json!( { "authorized": false, "user_name": String::default() } )
}

pub fn gen_token(user_name: &str) -> String {
    let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user_name);
    claims.sign_with_key(&key).unwrap()
}

pub fn read_token(token: &str) -> Result<String, String> {
    let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret").unwrap();
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)
        .map_err(|e| e.to_string())?;

    if claims.get("sub").is_some() {
        Ok(claims["sub"].clone())
    } else {
        Err("Token not valid".into())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for APIToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<APIToken, ()> {
        let cookies = request.cookies();
        let token = cookies.get("token");
        if token.is_none() { return Outcome::Forward(()); }
        match read_token(token.unwrap().value()) {
            Ok(claim) => Outcome::Success(APIToken(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}