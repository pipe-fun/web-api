use rocket_contrib::json::Json;
use crate::status::db_api::DbAPIStatus;
use crate::smtp;
use crate::my_trait::StatusTrait;
use crate::status::user::active::{ActiveStatus, _ActiveStatus};
use crate::user::tools;

#[derive(Serialize, Deserialize)]
pub struct ActiveCode {
    pub code: String,
    owner: String,
}

impl ActiveCode {
    pub fn new(code: String, owner: String) -> ActiveCode {
        ActiveCode {
            code,
            owner,
        }
    }

    pub fn to_db_and_email(&self, email: &str) -> Result<ActiveStatus, ActiveStatus> {
        if let Err(_) = smtp::send_email(email, &self.code) {
            return Err(ActiveStatus::default().set_status(_ActiveStatus::SendEmailError));
        }
        match tools::create_code(self) {
            Ok(()) => Ok(ActiveStatus::default()),
            Err(e) => Err(ActiveStatus::set_db_api_err_simple(e))
        }
    }
}

#[get("/active/<code>")]
pub fn active(code: String) -> Json<ActiveStatus> {
    let op = |owner: &str| -> Result<(), DbAPIStatus> {
        match tools::read_users() {
            Ok(mut users) => {
                let user = users
                    .iter_mut()
                    .find(|u| u.user_name.eq(owner)).unwrap();
                user.set_active(true);
                tools::update_user(user)
            }
            Err(e) => Err(e)
        }
    };

    let get_status = |ac: &ActiveCode| -> ActiveStatus {
        if let Err(e) = op(&ac.owner) {
            ActiveStatus::set_db_api_err_simple(e)
        } else {
            if let Err(e) = tools::delete_active_code(ac) {
                ActiveStatus::set_db_api_err_simple(e)
            } else {
                ActiveStatus::default()
            }
        }
    };

    match tools::read_active_code() {
        Err(e) => Json(ActiveStatus::set_db_api_err_simple(e)),
        Ok(v_ac) => {
            if let Some(ac) = v_ac
                .iter()
                .find(|a| a.code.eq(&code)) {
                Json(get_status(ac))
            } else {
                Json(ActiveStatus::default().set_status(_ActiveStatus::InvalidCode))
            }
        }
    }
}