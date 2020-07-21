use crate::user::user::User;
use crate::status::db_api::DBApiStatus;

#[derive(Serialize, Deserialize, Default)]
pub struct Data {
    id: i32,
    user_name: String,
}

impl Data {
    pub fn new(user: &User) -> Data {
        Data {
            id: user.id,
            user_name: user.user_name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum _Status {
    LoginSuccessfully,
    UserNameOrPasswordWrong,
    DBApiError,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    status_code: u8,
    login_status: _Status,
    db_api_status: DBApiStatus,
    data: Data,
}

impl Default for Status {
    fn default() -> Self {
        Status {
            status_code: 0,
            login_status: _Status::LoginSuccessfully,
            db_api_status: DBApiStatus::default(),
            data: Data::default()
        }
    }
}

impl Status {
    pub fn set_login_status(self, status: _Status) -> Self {
        Status {
            status_code: status as u8,
            login_status: status,
            ..self
        }
    }

    pub fn set_db_api_status(self, status: DBApiStatus) -> Self {
        Status {
            db_api_status: status,
            ..self
        }
    }

    pub fn set_data(self, data: Data) -> Self {
        Status {
            data,
            ..self
        }
    }
}