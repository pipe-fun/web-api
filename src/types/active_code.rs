use status_protoc::status::user::register::{RegisterStatus, _RegisterStatus};
use status_protoc::status::db_api::DbAPIStatus;
use status_protoc::my_trait::StatusTrait;
use crate::{smtp, request};
use crate::smtp::EmailType;

#[derive(Serialize, Deserialize)]
pub struct ActiveCode {
    code: String,
    owner: String,
}

impl ActiveCode {
    pub fn new(code: String, owner: String) -> ActiveCode {
        ActiveCode {
            code,
            owner,
        }
    }

    pub fn to_db_and_email(&self, email: &str) -> Result<RegisterStatus, RegisterStatus> {
        if let Err(_) = smtp::check_email(&email) {
            return Err(RegisterStatus::default().set_status(_RegisterStatus::InvalidEmailAddress));
        }
        if let Err(_) = smtp::send_email(email, EmailType::Active, &self.code) {
            return Err(RegisterStatus::default().set_status(_RegisterStatus::SendEmailError));
        }
        match create(self) {
            Ok(()) => Ok(RegisterStatus::default()),
            Err(e) => Err(RegisterStatus::set_db_api_err_simple(e))
        }
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn owner(&self) -> String {
        self.owner.clone()
    }
}

pub fn read_by_code(code: &str) -> Result<Vec<ActiveCode>, DbAPIStatus> {
    let url = format!("http://localhost:1122/active_code/read_by_code/{}", code);
    request::get(&url)
}

pub fn create(code: &ActiveCode) -> Result<(), DbAPIStatus> {
    request::post("http://localhost:1122/active_code/create", code)
}

pub fn delete(active_code: &ActiveCode) -> Result<(), DbAPIStatus> {
    let url = format!("http://localhost:1122/active_code/delete/{}", active_code.code());
    request::delete(&url)
}