use rocket_contrib::json::Json;
use rand::Rng;
use status_protoc::status::user::check::{CheckStatus, _CheckStatus};
use status_protoc::my_trait::StatusTrait;
use status_protoc::status::user::change::{ChangeStatus, _ChangeStatus};
use status_protoc::status::db_api::DbAPIStatus;
use crate::user::{check_code, user, tools};
use crate::user::user::User;
use crate::smtp;
use crate::user::check_code::CheckCode;

#[derive(Serialize, Deserialize)]
pub struct NewPassword {
    code: i32,
    new_password: String,
}

#[get("/send_code/<email>")]
pub fn send_check_code(email: String) -> Json<CheckStatus> {
    if let Err(_) = smtp::check_email(&email) {
        return Json(CheckStatus::default().set_status(_CheckStatus::InvalidEmailAddress));
    }

    let valid = |users: Vec<User>| -> Option<User> {
        users.into_iter()
            .filter(|u| u.active)
            .find(|u| u.user_email.eq(&email))
    };

    let op = |users: Vec<User>| -> CheckStatus {
        match valid(users) {
            Some(u) => {
                let mut rng = rand::thread_rng();
                let code = CheckCode::new(rng.gen_range(10000, 99999)
                                          , u.user_name.clone());
                if let Err(e) = code.to_db_and_email(&email) { e } else {
                    CheckStatus::default()
                }
            }
            None => CheckStatus::default()
        }
    };

    let status = match user::read() {
        Ok(users) => op(users),
        Err(e) => CheckStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[post("/update_password", format = "json", data = "<info>")]
pub fn update_password(info: Json<NewPassword>) -> Json<ChangeStatus> {
    if info.new_password.len() < 8 {
        return Json(ChangeStatus::default().set_status(_ChangeStatus::PasswordTooShort))
    }

    let op = |owner: &str| -> Result<(), DbAPIStatus> {
        match user::read() {
            Ok(mut users) => {
                let user = users
                    .iter_mut()
                    .filter(|u| u.active)
                    .find(|u| u.user_name.eq(owner)).unwrap();
                let password = tools::hash(&info.new_password);
                user.set_password(password);
                user::update(user)
            }
            Err(e) => Err(e)
        }
    };

    let get_status = |cc: &CheckCode| -> ChangeStatus {
        if let Err(e) = op(&cc.owner()) {
            ChangeStatus::set_db_api_err_simple(e)
        } else {
            if let Err(e) = check_code::delete(cc) {
                ChangeStatus::set_db_api_err_simple(e)
            } else {
                ChangeStatus::default()
            }
        }
    };

    match check_code::read() {
        Err(e) => Json(ChangeStatus::set_db_api_err_simple(e)),
        Ok(v_cc) => {
            if let Some(cc) = v_cc
                .iter()
                .find(|a| a.code().eq(&info.code)) {
                Json(get_status(cc))
            } else {
                Json(ChangeStatus::default().set_status(_ChangeStatus::InvalidCode))
            }
        }
    }
}