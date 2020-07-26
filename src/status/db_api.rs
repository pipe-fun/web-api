#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum _DbAPIStatus {
    Ok,
    ConnectRefused,
    DataError,
    DbError,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct DbAPIStatus {
    status_code: u8,
    status: _DbAPIStatus,
    message: String,
}

impl Default for DbAPIStatus {
    fn default() -> Self {
        DbAPIStatus {
            status_code: 0,
            status: _DbAPIStatus::Ok,
            message: "db api is fine".into(),
        }
    }
}

impl DbAPIStatus {
    pub fn new(status: _DbAPIStatus, message: String) -> Self {
        DbAPIStatus {
            status_code: status as u8,
            status,
            message,
        }
    }

    pub fn clone(db_api_status: &DbAPIStatus) -> Self {
        DbAPIStatus {
            status_code: db_api_status.status_code,
            status: db_api_status.status,
            message: db_api_status.message.clone()
        }
    }
}