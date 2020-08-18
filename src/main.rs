#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

mod user;
mod console;
mod smtp;

use rocket::Config;
use rocket::config::Environment;
use rocket_cors::{AllowedOrigins, Origins};
use std::collections::HashSet;

use crate::user::login::static_rocket_route_info_for_login;
use crate::user::auth::static_rocket_route_info_for_authorized;
use crate::user::auth::static_rocket_route_info_for_not_authorized;
use crate::user::register::static_rocket_route_info_for_register;
use crate::user::active::static_rocket_route_info_for_active;
use crate::user::new_password::static_rocket_route_info_for_send_check_code;
use crate::user::new_password::static_rocket_route_info_for_update_password;

use crate::console::test::static_rocket_route_info_for_test;
use crate::console::test::static_rocket_route_info_for_test_error;

fn rocket_web_api() -> rocket::Rocket {
    let mut config = Config::new(Environment::Development);
    config.set_address("127.0.0.1").unwrap();
    config.set_port(8888);

    let mut _origin = HashSet::new();
    let mut origin = Origins::default();
    _origin.insert("http://127.0.0.1:8080".to_string());
    origin.exact = Some(_origin);

    let cors_options = rocket_cors::CorsOptions::default()
        .allowed_origins(AllowedOrigins::Some(origin))
        .send_wildcard(false)
        .allow_credentials(true);

    let cors = rocket_cors::Cors::from_options(&cors_options).unwrap();

    rocket::custom(config)
        .mount("/user", routes![login, authorized, not_authorized, register, active, send_check_code, update_password])
        .mount("/console", routes![test, test_error])
        .attach(cors)
}

fn main() {
    rocket_web_api().launch();
}
