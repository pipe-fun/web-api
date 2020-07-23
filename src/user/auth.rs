use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use jwt::{VerifyWithKey, SignWithKey};
use hmac::{Hmac, NewMac};
use std::collections::BTreeMap;
use sha2::Sha256;
use chrono::{Utc, Duration, TimeZone};
use std::ops::Add;

pub struct ApiToken(pub String);

pub fn gen_token(user_name: &str) -> String {
    let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    let iat = Utc::now();
    let exp = iat.add(Duration::minutes(30)).format("%Y-%m-%d %H:%M:%S").to_string();
    let iat = iat.format("%Y-%m-%d %H:%M:%S").to_string();

    claims.insert("nbf", iat.clone());
    claims.insert("iat", iat);
    claims.insert("exp", exp);
    claims.insert("sub", user_name.into());

    claims.sign_with_key(&key).unwrap()
}

pub fn read_token(token: &str) -> Result<String, String> {
    let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret").unwrap();
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)
        .map_err(|e| e.to_string())?;

    if claims.get("exp").is_some() {
        let exp = Utc.datetime_from_str(&claims["exp"], "%Y-%m-%d %H:%M:%S")
            .map_err(|e| e.to_string())?;
        if exp < Utc::now() { return Err("Token not valid".into()); }
    } else {
        return Err("Token not valid".into());
    }

    if claims.get("sub").is_some() {
        Ok(claims["sub"].clone())
    } else {
        Err("Token not valid".into())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiToken, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.is_empty() {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiToken(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}