use std::collections::HashMap;
use status_protoc::status::user::check::{CheckStatus, _CheckStatus};
use status_protoc::status::db_api::{DbAPIStatus, _DbAPIStatus};
use status_protoc::my_trait::StatusTrait;
use crate::smtp;
use crate::user::check_code;
use crate::user::tools::check_response;
use crate::smtp::EmailType;

#[derive(Serialize, Deserialize)]
pub struct CheckCode {
    code: String,
    owner: String,
}

impl CheckCode {
    pub fn new(code: String, owner: String) -> CheckCode {
        CheckCode {
            code,
            owner,
        }
    }

    pub fn to_db_and_email(&self, email: &str) -> Result<CheckStatus, CheckStatus> {
        if let Err(_) = smtp::check_email(email) {
            return Err(CheckStatus::default().set_status(_CheckStatus::InvalidEmailAddress));
        }
        if let Err(_) = smtp::send_email(email, EmailType::ChangePassword, &self.code.to_string()) {
            return Err(CheckStatus::default().set_status(_CheckStatus::SendEmailError));
        }
        match check_code::create(self) {
            Ok(()) => Ok(CheckStatus::default()),
            Err(e) => Err(CheckStatus::set_db_api_err_simple(e))
        }
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn owner(&self) -> String {
        self.owner.clone()
    }
}


pub fn read() -> Result<Vec<CheckCode>, DbAPIStatus> {
    match reqwest::blocking::get("http://localhost:1122/api/user/check_code/read") {
        Ok(response) => {
            match response.json::<Vec<CheckCode>>() {
                Ok(check_code) => Ok(check_code),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn create(code: &CheckCode) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.post("http://localhost:1122/api/user/check_code/create").json(code).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn delete(code: &CheckCode) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/user/check_code/delete/{}", code.code());
    match client.delete(&uri).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}