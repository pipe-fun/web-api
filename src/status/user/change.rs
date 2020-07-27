use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::my_trait::StatusTrait;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum _ChangeStatus {
    ChangeSuccessfully,
    InvalidCode,
    DbAPIError,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeStatus {
    status_code: u8,
    status: _ChangeStatus,
    db_api_status: DbAPIStatus,
}

impl Default for ChangeStatus {
    fn default() -> Self {
        ChangeStatus {
            status_code: 0,
            status: _ChangeStatus::ChangeSuccessfully,
            db_api_status: DbAPIStatus::default(),
        }
    }
}

impl StatusTrait for ChangeStatus {
    type StatusCode = u8;
    type Status = _ChangeStatus;
    type DbAPIStatus = DbAPIStatus;
    type _DbAPIStatus = _DbAPIStatus;

    fn set_status(self, status: _ChangeStatus) -> Self {
        ChangeStatus {
            status_code: status as u8,
            status,
            ..self
        }
    }

    fn set_db_api_status(self, status: DbAPIStatus) -> Self {
        ChangeStatus {
            db_api_status: status,
            ..self
        }
    }

    fn set_db_api_err(status: Self::_DbAPIStatus, e: String) -> Self {
        ChangeStatus::default().set_status(_ChangeStatus::DbAPIError)
            .set_db_api_status(DbAPIStatus::new(status, e))
    }

    fn set_db_api_err_simple(status: Self::DbAPIStatus) -> Self {
        ChangeStatus::default().set_status(_ChangeStatus::DbAPIError)
            .set_db_api_status(status)
    }

    fn status_code(&self) -> Self::StatusCode {
        self.status_code
    }

    fn status(&self) -> _ChangeStatus {
        self.status
    }

    fn db_api_status(&self) -> Self::DbAPIStatus {
        DbAPIStatus::clone(&self.db_api_status)
    }
}