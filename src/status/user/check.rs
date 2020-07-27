use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::my_trait::StatusTrait;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum _CheckStatus {
    SendSuccessfully,
    InvalidEmailAddress,
    SendEmailError,
    DbAPIError,
}

#[derive(Serialize, Deserialize)]
pub struct CheckStatus {
    status_code: u8,
    status: _CheckStatus,
    db_api_status: DbAPIStatus,
}

impl Default for CheckStatus {
    fn default() -> Self {
        CheckStatus {
            status_code: 0,
            status: _CheckStatus::SendSuccessfully,
            db_api_status: DbAPIStatus::default(),
        }
    }
}

impl StatusTrait for CheckStatus {
    type StatusCode = u8;
    type Status = _CheckStatus;
    type DbAPIStatus = DbAPIStatus;
    type _DbAPIStatus = _DbAPIStatus;

    fn set_status(self, status: _CheckStatus) -> Self {
        CheckStatus {
            status_code: status as u8,
            status,
            ..self
        }
    }

    fn set_db_api_status(self, status: DbAPIStatus) -> Self {
        CheckStatus {
            db_api_status: status,
            ..self
        }
    }

    fn set_db_api_err(status: Self::_DbAPIStatus, e: String) -> Self {
        CheckStatus::default().set_status(_CheckStatus::DbAPIError)
            .set_db_api_status(DbAPIStatus::new(status, e))
    }

    fn set_db_api_err_simple(status: Self::DbAPIStatus) -> Self {
        CheckStatus::default().set_status(_CheckStatus::DbAPIError)
            .set_db_api_status(status)
    }

    fn status_code(&self) -> Self::StatusCode {
        self.status_code
    }

    fn status(&self) -> _CheckStatus {
        self.status
    }

    fn db_api_status(&self) -> Self::DbAPIStatus {
        DbAPIStatus::clone(&self.db_api_status)
    }
}