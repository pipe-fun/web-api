use rocket_contrib::json::Json;
use status_protoc::status::user::active::{ActiveStatus, _ActiveStatus};
use status_protoc::status::db_api::DbAPIStatus;
use status_protoc::my_trait::StatusTrait;
use crate::user::{user, active_code};
use crate::user::active_code::ActiveCode;

#[get("/active/<code>")]
pub fn active(code: String) -> Json<ActiveStatus> {
    let op = |owner: &str| -> Result<(), DbAPIStatus> {
        match user::read_by_name(owner) {
            Ok(mut users) => {
                let user = users.get_mut(0).unwrap();
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

    match active_code::read_by_code(&code) {
        Err(e) => Json(ActiveStatus::set_db_api_err_simple(e)),
        Ok(ac) => {
            if ac.is_empty() {
                Json(ActiveStatus::default().set_status(_ActiveStatus::InvalidCode))
            } else {
                Json(get_status(ac.get(0).unwrap()))
            }
        }
    }
}