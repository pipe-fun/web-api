#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum _DbAPIStatus {
    Ok,
    ConnectRefused,
    DataError,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct DbAPIStatus {
    status: _DbAPIStatus,
    message: String,
}

impl Default for DbAPIStatus {
    fn default() -> Self {
        DbAPIStatus {
            status: _DbAPIStatus::Ok,
            message: "db api is fine".into(),
        }
    }
}

impl DbAPIStatus {
    pub fn new(status: _DbAPIStatus, message: String) -> Self {
        DbAPIStatus {
            status,
            message,
        }
    }
}