use std::collections::HashMap;
use status_protoc::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::user::register::RegisterInfo;
use crate::user::tools::check_response;

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub active: bool,
}

impl User {
    pub fn new(new_user: &RegisterInfo) -> User {
        User {
            user_name: new_user.user_name.clone(),
            user_password: new_user.user_password.clone(),
            user_email: new_user.user_email.clone(),
            active: false
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_password(&mut self, password: String) {
        self.user_password = password;
    }
}

pub fn create(info: &RegisterInfo) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.post("http://localhost:1122/api/user/create").json(&User::new(info)).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
    }
}

pub fn read() -> Result<Vec<User>, DbAPIStatus> {
    match reqwest::blocking::get("http://localhost:1122/api/user/read") {
        Ok(response) => {
            match response.json::<Vec<User>>() {
                Ok(users) => Ok(users),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn update(user: &User) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/user/update/{}", user.user_name);
    match client.put(&uri).json(user).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}
