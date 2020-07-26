use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::my_trait::StatusTrait;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum _ActiveStatus {
    ActiveSuccessfully,
    SendEmailError,
    InvalidCode,
    DbAPIError,
}

#[derive(Serialize, Deserialize)]
pub struct ActiveStatus {
    status_code: u8,
    status: _ActiveStatus,
    db_api_status: DbAPIStatus,
}

impl Default for ActiveStatus {
    fn default() -> Self {
        ActiveStatus {
            status_code: 0,
            status: _ActiveStatus::ActiveSuccessfully,
            db_api_status: DbAPIStatus::default(),
        }
    }
}

impl StatusTrait for ActiveStatus {
    type StatusCode = u8;
    type Status = _ActiveStatus;
    type DbAPIStatus = DbAPIStatus;
    type _DbAPIStatus = _DbAPIStatus;

    fn set_status(self, status: _ActiveStatus) -> Self {
        ActiveStatus {
            status_code: status as u8,
            status,
            ..self
        }
    }

    fn set_db_api_status(self, status: DbAPIStatus) -> Self {
        ActiveStatus {
            db_api_status: status,
            ..self
        }
    }

    fn set_db_api_err(status: Self::_DbAPIStatus, e: String) -> Self {
        ActiveStatus::default().set_status(_ActiveStatus::DbAPIError)
            .set_db_api_status(DbAPIStatus::new(status, e))
    }

    fn set_db_api_err_simple(status: Self::DbAPIStatus) -> Self {
        ActiveStatus::default().set_status(_ActiveStatus::DbAPIError)
            .set_db_api_status(status)
    }

    fn status_code(&self) -> Self::StatusCode {
        self.status_code
    }

    fn status(&self) -> _ActiveStatus {
        self.status
    }

    fn db_api_status(&self) -> Self::DbAPIStatus {
        DbAPIStatus::clone(&self.db_api_status)
    }
}