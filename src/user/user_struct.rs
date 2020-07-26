use crate::user::register::RegisterInfo;

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub active: bool,
}

impl User {
    pub fn new(new_user: &RegisterInfo) -> User {
        User {
            user_name: new_user.user_name.clone(),
            user_password: new_user.user_password.clone(),
            user_email: new_user.user_email.clone(),
            active: false
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}