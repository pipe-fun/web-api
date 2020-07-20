use rocket_contrib::json::Json;
use rocket::local::Client;
use crate::db::user::User;


#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    user_name: String,
    user_password: String,
}

impl LoginInfo {
    fn equal(&self, user: &User) -> bool {
        user.user_name.eq(&self.user_name)
        && user.user_password.eq(&self.user_password)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Data {
    id: i32,
    user_name: String
}

impl Data {
    pub fn new(user: &User) -> Data {
        Data {
            id: user.id,
            user_name: user.user_name.clone()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum _Status {
    LoginSuccessfully,
    UserNameOrPasswordWrong
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    message: String,
    data: Data
}

impl Status {
    pub fn new(message: _Status, data: Data) -> Status {
        let message = match message {
            _Status::LoginSuccessfully => {
                format!("login successfully")
            }
            _Status::UserNameOrPasswordWrong => {
                format!("user name or password wrong")
            }
        };

        Status {
            message,
            data
        }
    }
}

#[post("/login", format = "json", data = "<info>")]
pub fn login(info: Json<LoginInfo>) -> Result<Json<Status>, String> {
    let uri = "/db/user/read";
    let client = Client::new(crate::rocket_db()).unwrap();
    let response = client.get(uri).dispatch().body_string().unwrap();

    let users: Vec<User> = serde_json::from_str(&response).unwrap();
    let status = if let Some(u) = users.iter().find(|&u| info.equal(u)) {
        Status::new(_Status::LoginSuccessfully, Data::new(u))
    } else {
        Status::new(_Status::UserNameOrPasswordWrong, Data::default())
    };

    Ok(Json(status))
}
