use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    user_registered_time: NaiveDateTime,
    user_recently_login_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    user_registered_time: NaiveDateTime,
    user_recently_login_time: NaiveDateTime,
}