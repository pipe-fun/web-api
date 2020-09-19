use status_protoc::status::db_api::{DbAPIStatus, _DbAPIStatus};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use dotenv_codegen::dotenv;
use crate::user::tools::check_response;

const API_ROOT: &str = dotenv!("DB_API");
pub const COOKIE_DOMAIN: &str = dotenv!("COOKIE_DOMAIN");
pub const COOKIE_MAX_AGE: &str = dotenv!("COOKIE_MAX_AGE");

pub fn get_all<T: DeserializeOwned>(url: &str) -> Result<Vec<T>, DbAPIStatus> {
    let url = format!("{}{}", API_ROOT, url);
    match reqwest::blocking::get(&url) {
        Ok(response) => {
            match response.json::<Vec<T>>() {
                Ok(data) => Ok(data),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn get<T: DeserializeOwned>(url: &str) -> Result<Vec<T>, DbAPIStatus> {
    let url = format!("{}{}", API_ROOT, url);
    match reqwest::blocking::get(&url) {
        Ok(response) => {
            match response.json::<T>() {
                Ok(data) => Ok(vec![data]),
                Err(_) => Ok(vec![])
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn post<T: Serialize>(url: &str, data: &T) -> Result<(), DbAPIStatus> {
    let url = format!("{}{}", API_ROOT, url);
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.post(&url).json(data).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
    }
}

pub fn put<T: Serialize>(url: &str, data: &T) -> Result<(), DbAPIStatus> {
    let url = format!("{}{}", API_ROOT, url);
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.put(&url).json(data).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn delete(url: &str) -> Result<(), DbAPIStatus> {
    let url = format!("{}{}", API_ROOT, url);
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.delete(&url).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}