use rocket_contrib::json::Json;
use rocket::http::{Cookie, Cookies};
use rocket::http::SameSite;
use crate::user::{tools, auth};
use crate::status::user::login::{LoginStatus, _LoginStatus, Data};
use crate::user::user_struct::User;
use crate::my_trait::StatusTrait;

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
            .expires(time::now())
            .max_age(time::Duration::minutes(1))
            .path("/")
            .same_site(SameSite::Strict)
            .http_only(true)
            .finish();
        cookies.add(cookie);
    };

    let mut op = |users: Vec<User>| -> LoginStatus {
        if let Some(u) = users
            .into_iter()
            .filter(|u| u.active)
            .find(|u| info.equal(u)) {

            let token = auth::gen_token(&info.user_name);
            gen_cookie(&token);
            LoginStatus::default().set_data(Data::new(&u))
        } else {
            LoginStatus::default().set_status(_LoginStatus::UserNameOrPasswordWrongOrNoActive)
        }
    };

    let status = match tools::read_users() {
        Ok(u) => op(u),
        Err(e) => LoginStatus::set_db_api_err_simple(e)
    };

    Json(status)
}