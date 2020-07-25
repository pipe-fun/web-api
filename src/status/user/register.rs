use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::my_trait::StatusTrait;

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct RegisterStatus {
    status_code: u8,
    status: _RegisterStatus,
    db_api_status: DbAPIStatus,
}

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum _RegisterStatus {
    RegisterSuccessfully,
    UserNameHasExisted,
    EmailHasExisted,
    PasswordTooShort,
    UserNameTooShort,
    InvalidEmailAddress,
    SendEmailError,
    DbAPIError,
}

impl Default for RegisterStatus {
    fn default() -> Self {
        RegisterStatus {
            status_code: 0,
            status: _RegisterStatus::RegisterSuccessfully,
            db_api_status: DbAPIStatus::default(),
        }
    }
}

impl StatusTrait for RegisterStatus {
    type StatusCode = u8;
    type Status = _RegisterStatus;
    type DbAPIStatus = DbAPIStatus;
    type _DbAPIStatus = _DbAPIStatus;

    fn set_status(self, status: _RegisterStatus) -> Self {
        RegisterStatus {
            status_code: status as u8,
            status,
            ..self
        }
    }

    fn set_db_api_status(self, status: DbAPIStatus) -> Self {
        RegisterStatus {
            db_api_status: status,
            ..self
        }
    }

    fn set_db_api_err(status: Self::_DbAPIStatus, e: String) -> Self {
        RegisterStatus::default().set_status(_RegisterStatus::DbAPIError)
            .set_db_api_status(DbAPIStatus::new(status, e))
    }

    fn status_code(&self) -> Self::StatusCode {
        self.status_code
    }

    fn status(&self) -> Self::Status {
        self.status
    }

    fn db_api_status(&self) -> Self::DbAPIStatus {
        DbAPIStatus::clone(&self.db_api_status)
    }
}