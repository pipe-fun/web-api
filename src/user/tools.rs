use argon2::{self, Config};
use rand::Rng;
use crate::user::user_struct::User;
use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};

pub fn hash(password: &str) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}

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