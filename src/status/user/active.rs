use crate::status::db_api::DbAPIStatus;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum _ActiveStatus {
    Successfully,
    SendEmailError,
    DbAPIError,
}

#[derive(Serialize, Deserialize)]
pub struct ActiveStatus {
    status_code: u8,
    status: _ActiveStatus,
    pub db_api_status: DbAPIStatus,
}

impl Default for ActiveStatus {
    fn default() -> Self {
        ActiveStatus {
            status_code: 0,
            status: _ActiveStatus::Successfully,
            db_api_status: DbAPIStatus::default(),
        }
    }
}

impl ActiveStatus {
    pub fn set_active_status(self, status: _ActiveStatus) -> Self {
        ActiveStatus {
            status_code: status as u8,
            status,
            ..self
        }
    }

    pub fn set_db_api_status(self, status: DbAPIStatus) -> Self {
        ActiveStatus {
            db_api_status: status,
            ..self
        }
    }

    pub fn status(&self) -> _ActiveStatus {
        self.status
    }
}