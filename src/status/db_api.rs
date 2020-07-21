#[derive(Serialize, Deserialize)]
pub enum _DBApiStatus {
    Ok,
    ConnectRefused,
    DataError,
}

#[derive(Serialize, Deserialize)]
pub struct DBApiStatus {
    status: _DBApiStatus,
    message: String,
}

impl Default for DBApiStatus {
    fn default() -> Self {
        DBApiStatus {
            status: _DBApiStatus::Ok,
            message: "db api is fine".into(),
        }
    }
}

impl DBApiStatus {
    pub fn new(status: _DBApiStatus, message: String) -> Self {
        DBApiStatus {
            status,
            message,
        }
    }
}