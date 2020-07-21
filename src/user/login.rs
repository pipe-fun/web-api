use rocket_contrib::json::Json;
use crate::user::tools;
use crate::status::user::login::{LoginStatus, _LoginStatus, Data};
use crate::user::user_struct::User;

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    user_name: String,
    user_password: String,
}

impl LoginInfo {
    fn equal(&self, user: &User) -> bool {
        user.user_name.eq(&self.user_name)
            && user.user_password.eq(&self.user_password)
    }
}

#[post("/login", format = "json", data = "<info>")]
pub fn login(info: Json<LoginInfo>) -> Json<LoginStatus> {
    let op = |users: Vec<User>| -> LoginStatus {
        if let Some(u) = users.iter().find(|&u| info.equal(u)) {
            LoginStatus::default().set_data(Data::new(u))
        } else {
            LoginStatus::default().set_login_status(_LoginStatus::UserNameOrPasswordWrong)
        }
    };

    let status = match tools::read_users() {
        Ok(u) => { op(u) }
        Err(e) => {
            LoginStatus::default().set_login_status(_LoginStatus::DbAPIError).
                set_db_api_status(e)
        }
    };

    Json(status)
}