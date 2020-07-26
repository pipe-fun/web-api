use rand::Rng;
use argon2::{self, Config, ThreadMode};
use std::collections::HashMap;
use crate::user::user_struct::User;
use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::user::active::ActiveCode;
use crate::user::register::RegisterInfo;

pub fn hash(password: &str) -> String {
    let cpus = num_cpus::get();
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let mut config = Config::default();
    config.time_cost = 1;
    if cpus > 1 {
        config.lanes = cpus as u32;
        config.thread_mode = ThreadMode::Parallel;
    };
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}

fn op(status: HashMap<String, String>) -> Result<(), DbAPIStatus> {
    let status = status.get("status").unwrap();
    if status.eq("ok") {
        Ok(())
    } else {
        Err(DbAPIStatus::new(_DbAPIStatus::DbError, status.clone()))
    }
}

pub fn create_user(info: &RegisterInfo) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.post("http://localhost:1122/api/user/create").json(&User::new(info)).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => op(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
    }
}

pub fn read_users() -> Result<Vec<User>, DbAPIStatus> {
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

pub fn update_user(user: &User) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/user/update/{}", user.user_name);
    match client.put(&uri).json(user).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => op(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn read_active_code() -> Result<Vec<ActiveCode>, DbAPIStatus> {
    match reqwest::blocking::get("http://localhost:1122/api/user/active_code/read") {
        Ok(response) => {
            match response.json::<Vec<ActiveCode>>() {
                Ok(active_code) => Ok(active_code),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn create_code(active_code: &ActiveCode) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.post("http://localhost:1122/api/user/active_code/create").json(active_code).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => op(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn delete_active_code(active_code: &ActiveCode) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/user/active_code/delete/{}", active_code.code);
    match client.delete(&uri).json(active_code).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => op(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}