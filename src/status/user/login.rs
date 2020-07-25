use crate::status::db_api::DbAPIStatus;
use crate::user::user_struct::User;

#[derive(Serialize, Deserialize, Default)]
pub struct Data { user_name: String, }

impl Data {
    pub fn new(user: &User) -> Data {
        Data { user_name: user.user_name.clone(), }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum _LoginStatus {
    LoginSuccessfully,
    UserNameOrPasswordWrongOrNoActive,
    DbAPIError,
}

#[derive(Serialize, Deserialize)]
pub struct LoginStatus {
    status_code: u8,
    status: _LoginStatus,
    data: Data,
    db_api_status: DbAPIStatus,
}

impl Default for LoginStatus {
    fn default() -> Self {
        LoginStatus {
            status_code: 0,
            status: _LoginStatus::LoginSuccessfully,
            db_api_status: DbAPIStatus::default(),
            data: Data::default()
        }
    }
}

impl LoginStatus {
    pub fn set_login_status(self, status: _LoginStatus) -> Self {
        LoginStatus {
            status_code: status as u8,
            status,
            ..self
        }
    }

    pub fn set_db_api_status(self, status: DbAPIStatus) -> Self {
        LoginStatus {
            db_api_status: status,
            ..self
        }
    }

    pub fn set_data(self, data: Data) -> Self {
        LoginStatus {
            data,
            ..self
        }
    }
}