use rocket_contrib::json::Json;
use std::collections::HashMap;
use crate::user::tools;
use crate::status::user::register::{RegisterStatus, _RegisterStatus};
use crate::status::db_api::{_DbAPIStatus, DbAPIStatus};
use crate::user::user_struct::User;
use crate::smtp;
use crate::user::active::ActiveCode;
use crate::status::user::active::_ActiveStatus;
use crate::my_trait::StatusTrait;

#[derive(Serialize, Deserialize)]
pub struct RegisterInfo {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
}

pub fn check_rules(users: Vec<User>, info: &Json<RegisterInfo>) -> RegisterStatus {
    let f_u: Vec<User> = users.into_iter().filter(|u|
        u.user_name.eq(&info.user_name) || u.user_email.eq(&info.user_email)).collect();

    if !f_u.is_empty() {
        if f_u[0].user_name.eq(&info.user_name) {
            RegisterStatus::default().set_status(_RegisterStatus::UserNameHasExisted)
        } else {
            RegisterStatus::default().set_status(_RegisterStatus::EmailHasExisted)
        }
    } else if info.user_password.len() < 8 {
        RegisterStatus::default().set_status(_RegisterStatus::PasswordTooShort)
    } else if info.user_name.len() < 4 {
        RegisterStatus::default().set_status(_RegisterStatus::UserNameTooShort)
    } else if let Err(_) = smtp::check_email(&info.user_email) {
        RegisterStatus::default().set_status(_RegisterStatus::InvalidEmailAddress)
    } else {
        let ac = ActiveCode::new("code".into(), info.user_name.clone());
        if let Err(s) = ac.to_db_and_email(&info.user_email) {
            match s.status() {
                _ActiveStatus::SendEmailError => {
                    RegisterStatus::default().set_status(_RegisterStatus::SendEmailError)
                }
                _ActiveStatus::DbAPIError => {
                    RegisterStatus::default().set_db_api_status(s.db_api_status())
                }
                _ActiveStatus::Successfully => {
                    RegisterStatus::default()
                }
            }
        } else {
            RegisterStatus::default()
        }
    }
}

#[post("/register", format = "json", data = "<info>")]
pub fn register(mut info: Json<RegisterInfo>) -> Json<RegisterStatus> {
    let status = match tools::read_users() {
        Ok(u) => { check_rules(u, &info) }
        Err(e) => {
            RegisterStatus::default().set_status(_RegisterStatus::DbAPIError)
                .set_db_api_status(e)
        }
    };

    if status.eq(&RegisterStatus::default()) {
        info.user_password = tools::hash(&info.user_password);
    } else {
        return Json(status);
    }

    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let op = |status: &HashMap<String, String>| -> RegisterStatus {
        let status = status.get("status").unwrap();
        if status.eq("ok") {
            RegisterStatus::default()
        } else {
            RegisterStatus::default().set_status(_RegisterStatus::DbAPIError)
                .set_db_api_status(DbAPIStatus::new(_DbAPIStatus::DbError, status.clone()))
        }
    };

    let status = match client.post("http://localhost:1122/api/user/create")
        .json(&User::new(&info)).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => { op(&status) }
                Err(e) => {
                    RegisterStatus::set_db_api_err(_DbAPIStatus::DataError, e.to_string())
                }
            }
        }
        Err(e) => {
            RegisterStatus::set_db_api_err(_DbAPIStatus::ConnectRefused, e.to_string())
        }
    };

    Json(status)
}