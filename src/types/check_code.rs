use status_protoc::status::user::check::{CheckStatus, _CheckStatus};
use status_protoc::status::db_api::DbAPIStatus;
use status_protoc::my_trait::StatusTrait;
use crate::{smtp, request};
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
        match create(self) {
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
    request::get_all("/check_code/read")
}

pub fn create(code: &CheckCode) -> Result<(), DbAPIStatus> {
    request::post("/check_code/create", code)
}

pub fn delete(code: &CheckCode) -> Result<(), DbAPIStatus> {
    let url = format!("/check_code/delete/{}", code.code());
    request::delete(&url)
}