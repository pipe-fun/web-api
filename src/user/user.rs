use status_protoc::status::db_api::DbAPIStatus;
use crate::user::register::RegisterInfo;
use crate::request;

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub active: bool,
}

impl User {
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_password(&mut self, password: String) {
        self.user_password = password;
    }
}

pub fn create(info: &RegisterInfo) -> Result<(), DbAPIStatus> {
    request::post("http://localhost:1122/user/create", info)
}

pub fn read() -> Result<Vec<User>, DbAPIStatus> {
    request::get_all("http://localhost:1122/user/read")
}

pub fn read_by_name(name: &str) -> Result<Vec<User>, DbAPIStatus> {
    let url = format!("http://localhost:1122/user/read_by_name/{}", name);
    request::get(&url)
}

pub fn update(user: &User) -> Result<(), DbAPIStatus> {
    let url = format!("http://localhost:1122/user/update/{}", user.user_name);
    request::put(&url, user)
}
