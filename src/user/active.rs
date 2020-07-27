use rocket_contrib::json::Json;
use crate::user::{user, active_code};
use crate::status::user::active::{ActiveStatus, _ActiveStatus};
use crate::status::db_api::DbAPIStatus;
use crate::my_trait::StatusTrait;
use crate::user::active_code::ActiveCode;

#[get("/active/<code>")]
pub fn active(code: String) -> Json<ActiveStatus> {
    let op = |owner: &str| -> Result<(), DbAPIStatus> {
        match user::read() {
            Ok(mut users) => {
                let user = users
                    .iter_mut()
                    .find(|u| u.user_name.eq(owner)).unwrap();
                user.set_active(true);
                user::update(user)
            }
            Err(e) => Err(e)
        }
    };

    let get_status = |ac: &ActiveCode| -> ActiveStatus {
        if let Err(e) = op(&ac.owner()) {
            ActiveStatus::set_db_api_err_simple(e)
        } else {
            if let Err(e) = active_code::delete(ac) {
                ActiveStatus::set_db_api_err_simple(e)
            } else {
                ActiveStatus::default()
            }
        }
    };

    match active_code::read() {
        Err(e) => Json(ActiveStatus::set_db_api_err_simple(e)),
        Ok(v_ac) => {
            if let Some(ac) = v_ac
                .iter()
                .find(|a| a.code().eq(&code)) {
                Json(get_status(ac))
            } else {
                Json(ActiveStatus::default().set_status(_ActiveStatus::InvalidCode))
            }
        }
    }
}