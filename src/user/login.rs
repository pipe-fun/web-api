use rocket_contrib::json::Json;
use crate::user::user::User;
use crate::status::login::{Status, _Status, Data};
use crate::status::db_api::{DBApiStatus, _DBApiStatus};


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
pub fn login(info: Json<LoginInfo>) -> Json<Status> {
    let set_db_api_err = |status: _DBApiStatus, e: String| {
        Status::default().set_login_status(_Status::DBApiError).
            set_db_api_status(DBApiStatus::new(status, e))
    };

    let login_op = |users: Vec<User>| {
        if let Some(u) = users.iter().find(|&u| info.equal(u)) {
            Status::default().set_data(Data::new(u))
        } else {
            Status::default().set_login_status(_Status::UserNameOrPasswordWrong)
        }
    };

    let status = match reqwest::blocking::get("http://localhost:1122/db/user/read") {
        Ok(response) => {
            match response.json::<Vec<User>>() {
                Ok(users) => { login_op(users) }
                Err(e) => { set_db_api_err(_DBApiStatus::DataError, e.to_string()) }
            }
        }
        Err(e) => { set_db_api_err(_DBApiStatus::ConnectRefused, e.to_string()) }
    };

    Json(status)
}