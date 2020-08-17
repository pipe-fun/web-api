use rand::Rng;
use argon2::{self, Config, ThreadMode};
use std::collections::HashMap;
use status_protoc::status::db_api::{DbAPIStatus, _DbAPIStatus};

pub fn hash(password: &str) -> String {
    let cpus = num_cpus::get();
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let mut config = Config::default();
    config.time_cost = 1;
    if cpus > 1 {
        config.lanes = cpus as u32;
        config.thread_mode = ThreadMode::Parallel;
    };
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

pub fn verify(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}

pub fn check_response(status: HashMap<String, String>) -> Result<(), DbAPIStatus> {
    let status = status.get("status").unwrap();
    if status.eq("ok") {
        Ok(())
    } else {
        Err(DbAPIStatus::new(_DbAPIStatus::DbError, status.clone()))
    }
}