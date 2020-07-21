use rocket_contrib::json::Json;
use std::collections::HashMap;
use crate::user::tools;
use crate::status::user::register::{RegisterStatus, _RegisterStatus};
use crate::status::db_api::{_DbAPIStatus, DbAPIStatus};
use crate::user::user_struct::{NewUser, User};

#[post("/register", format = "json", data = "<info>")]
pub fn register(info: Json<NewUser>) -> Json<RegisterStatus> {
    let check = |users: Vec<User>| -> RegisterStatus {
        let f_u: Vec<User> = users.into_iter().filter(|u|
            u.user_name.eq(&info.user_name) || u.user_email.eq(&info.user_email)).collect();

        if !f_u.is_empty() {
            if f_u.get(0).unwrap().user_name.eq(&info.user_name) {
                RegisterStatus::default().set_register_status(_RegisterStatus::UserNameHasExisted)
            } else {
                RegisterStatus::default().set_register_status(_RegisterStatus::EmailHasExisted)
            }
        } else if info.user_password.len() < 8 {
            RegisterStatus::default().set_register_status(_RegisterStatus::PasswordTooShort)
        } else {
            RegisterStatus::default()
        }
    };

    let status = match tools::read_users() {
        Ok(u) => { check(u) }
        Err(e) => {
            RegisterStatus::default().set_register_status(_RegisterStatus::DbAPIError).
                set_db_api_status(e)
        }
    };

    if !status.eq(&RegisterStatus::default()) { return Json(status); }

    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let op = |status: &HashMap<String, String>| -> RegisterStatus {
        if status.get("status").unwrap().eq("ok") {
            RegisterStatus::default()
        } else {
            RegisterStatus::default().set_register_status(_RegisterStatus::UndefinedError)
        }
    };

    let set_db_api_err = |status: _DbAPIStatus, e: String| -> RegisterStatus {
        RegisterStatus::default().set_register_status(_RegisterStatus::DbAPIError).
            set_db_api_status(DbAPIStatus::new(status, e))
    };

    let status = match client.post("http://localhost:1122/db/user/create").json(&info.into_inner()).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => { op(&status) }
                Err(e) => { set_db_api_err(_DbAPIStatus::DataError, e.to_string()) }
            }
        }
        Err(e) => {
            set_db_api_err(_DbAPIStatus::ConnectRefused, e.to_string())
        }
    };

    Json(status)
}