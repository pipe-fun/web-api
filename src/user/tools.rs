use crate::user::user_struct::User;
use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};

pub fn read_users() -> Result<Vec<User>, DbAPIStatus> {
    let status = match reqwest::blocking::get("http://localhost:1122/db/user/read") {
        Ok(response) => {
            match response.json::<Vec<User>>() {
                Ok(users) => { Ok(users) }
                Err(e) => {
                    Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
                }
            }
        }
        Err(e) => {
            Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
        }
    };

    status
}