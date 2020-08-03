use rocket_contrib::json::Json;
use crate::user::{tools, user};
use crate::status::user::register::{RegisterStatus, _RegisterStatus};
use crate::user::user::User;
use crate::user::active_code::ActiveCode;
use crate::status::user::active::_ActiveStatus;
use crate::my_trait::StatusTrait;

#[derive(Serialize, Deserialize)]
pub struct RegisterInfo {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
}

pub fn check_rules(users: Vec<User>, info: &Json<RegisterInfo>) -> RegisterStatus {
    let f_u: Vec<User> = users.into_iter().filter(|u|
        u.user_name.eq(&info.user_name) || u.user_email.eq(&info.user_email)).collect();

    let check_name_email = || -> RegisterStatus {
        if f_u[0].user_name.eq(&info.user_name) {
            RegisterStatus::default().set_status(_RegisterStatus::UserNameHasExisted)
        } else {
            RegisterStatus::default().set_status(_RegisterStatus::EmailHasExisted)
        }
    };

    let check_to_email = || -> RegisterStatus {
        let ac = ActiveCode::new("code".into(), info.user_name.clone());
        if let Err(s) = ac.to_db_and_email(&info.user_email) {
            match s.status() {
                _ActiveStatus::SendEmailError =>
                    RegisterStatus::default().set_status(_RegisterStatus::SendEmailError),
                _ActiveStatus::InvalidEmailAddress =>
                    RegisterStatus::default().set_status(_RegisterStatus::InvalidEmailAddress),
                _ActiveStatus::DbAPIError =>
                    RegisterStatus::set_db_api_err_simple(s.db_api_status()),
                _ => RegisterStatus::default()
            }
        } else {
            RegisterStatus::default()
        }
    };

    if !f_u.is_empty() {
        check_name_email()
    } else if info.user_password.len() < 8 {
        RegisterStatus::default().set_status(_RegisterStatus::PasswordTooShort)
    } else if info.user_name.len() < 4 {
        RegisterStatus::default().set_status(_RegisterStatus::UserNameTooShort)
    } else {
        check_to_email()
    }
}

#[post("/register", format = "json", data = "<info>")]
pub fn register(mut info: Json<RegisterInfo>) -> Json<RegisterStatus> {
    let status = match user::read() {
        Ok(u) => check_rules(u, &info),
        Err(e) => RegisterStatus::set_db_api_err_simple(e)
    };

    if status.eq(&RegisterStatus::default()) {
        info.user_password = tools::hash(&info.user_password);
    } else {
        return Json(status);
    }

    let status = match user::create(&info.into_inner()) {
        Ok(()) => RegisterStatus::default(),
        Err(e) => RegisterStatus::set_db_api_err_simple(e)
    };

    Json(status)
}