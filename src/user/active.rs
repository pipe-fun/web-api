use std::collections::HashMap;
use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::smtp;
use crate::my_trait::StatusTrait;
use crate::status::user::active::{ActiveStatus, _ActiveStatus};

#[derive(Serialize, Deserialize)]
pub struct ActiveCode {
    code: String,
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
        let op = |status: &HashMap<String, String>| -> ActiveStatus {
            let status = status.get("status").unwrap();
            if status.eq("ok") {
                ActiveStatus::default()
            } else {
                ActiveStatus::default().set_status(_ActiveStatus::DbAPIError)
                    .set_db_api_status(DbAPIStatus::new(_DbAPIStatus::DbError
                                                        , status.clone()))
            }
        };

        if let Err(_) = smtp::send_email(email, &self.code) {
            return Err(ActiveStatus::default().set_status(_ActiveStatus::SendEmailError));
        }

        let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
        match client.post("http://localhost:1122/api/user/active_code/create")
            .json(&self).send() {
            Ok(response) => {
                match response.json::<HashMap<String, String>>() {
                    Ok(status) => { Ok(op(&status)) }
                    Err(e) => {
                        Err(ActiveStatus::set_db_api_err(_DbAPIStatus::DataError, e.to_string()))
                    }
                }
            }
            Err(e) => {
                Err(ActiveStatus::set_db_api_err(_DbAPIStatus::ConnectRefused, e.to_string()))
            }
        }
    }
}