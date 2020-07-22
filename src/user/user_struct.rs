#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
}