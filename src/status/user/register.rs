use crate::status::db_api::DbAPIStatus;

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct RegisterStatus {
    status_code: u8,
    register_status: _RegisterStatus,
    db_api_status: DbAPIStatus,
}

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum _RegisterStatus {
    RegisterSuccessfully,
    UserNameHasExisted,
    EmailHasExisted,
    PasswordTooShort,
    UserNameTooShort,
    DbAPIError,
    UndefinedError,
}

impl Default for RegisterStatus {
    fn default() -> Self {
        RegisterStatus {
            status_code: 0,
            register_status: _RegisterStatus::RegisterSuccessfully,
            db_api_status: DbAPIStatus::default(),
        }
    }
}

impl RegisterStatus {
    pub fn set_register_status(self, status: _RegisterStatus) -> Self {
        RegisterStatus {
            status_code: status as u8,
            register_status: status,
            ..self
        }
    }

    pub fn set_db_api_status(self, status: DbAPIStatus) -> Self {
        RegisterStatus {
            db_api_status: status,
            ..self
        }
    }
}