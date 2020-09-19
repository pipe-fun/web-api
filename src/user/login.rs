use rocket_contrib::json::Json;
use rocket::http::{Cookie, Cookies};
use rocket::http::SameSite;
use status_protoc::status::user::login::{LoginStatus, Data, _LoginStatus};
use status_protoc::my_trait::StatusTrait;
use std::str::FromStr;
use crate::user::{tools, auth};
use crate::types::user::User;
use crate::types::user;
use crate::request::{COOKIE_DOMAIN, COOKIE_MAX_AGE};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    user_name: String,
    user_password: String,
}

impl LoginInfo {
    fn equal(&self, user: &User) -> bool {
        user.user_name.eq(&self.user_name)
            && tools::verify(&user.user_password, &self.user_password)
    }
}

#[post("/login", format = "json", data = "<info>")]
pub fn login(mut cookies: Cookies<'_>, info: Json<LoginInfo>) -> Json<LoginStatus> {
    let mut gen_cookie = |token: &str| {
        let cookie = Cookie::build("token", token.to_string())
            .domain(COOKIE_DOMAIN)
            .expires(time::now())
            .max_age(time::Duration::seconds(i64::from_str(COOKIE_MAX_AGE).unwrap_or_else(|_| 3600)))
            .path("/")
            .same_site(SameSite::Strict)
            .http_only(true)
            .finish();
        cookies.add(cookie);
    };

    let mut op = |user: Vec<User>| -> LoginStatus {
        if let Some(u) = user.into_iter()
            .filter(|u| u.active)
            .find(|u| info.equal(u)) {

            let token = auth::gen_token(&info.user_name);
            gen_cookie(&token);
            LoginStatus::default().set_data(Data::new(&u.user_name))
        } else {
            LoginStatus::default().set_status(_LoginStatus::UserNameOrPasswordWrongOrNoActive)
        }
    };

    let status = match user::read_by_name(&info.user_name) {
        Ok(u) => op(u),
        Err(e) => LoginStatus::set_db_api_err_simple(e)
    };

    Json(status)
}