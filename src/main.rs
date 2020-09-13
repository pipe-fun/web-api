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
mod request;
mod types;

use rocket::Config;
use rocket::config::Environment;
use rocket_cors::{AllowedOrigins, Origins};
use std::collections::HashSet;
use std::net::TcpStream;
use std::io::Write;
use std::env;
use std::str::FromStr;
use dotenv::dotenv;

use crate::user::login::static_rocket_route_info_for_login;
use crate::user::logout::static_rocket_route_info_for_logout;
use crate::user::auth::static_rocket_route_info_for_authorized;
use crate::user::auth::static_rocket_route_info_for_not_authorized;
use crate::user::register::static_rocket_route_info_for_register;
use crate::user::active::static_rocket_route_info_for_active;
use crate::user::reset_password::static_rocket_route_info_for_send_check_code;
use crate::user::reset_password::static_rocket_route_info_for_update_password;

use crate::console::task::static_rocket_route_info_for_task_read;
use crate::console::task::static_rocket_route_info_for_task_create;
use crate::console::task::static_rocket_route_info_for_task_update;
use crate::console::task::static_rocket_route_info_for_task_delete;
use crate::console::task::static_rocket_route_info_for_task_execute;
use crate::console::task::static_rocket_route_info_for_task_reload;

use crate::console::device::static_rocket_route_info_for_device_read;
use crate::console::device::static_rocket_route_info_for_device_create;
use crate::console::device::static_rocket_route_info_for_device_update;
use crate::console::device::static_rocket_route_info_for_device_delete;

fn rocket_web_api() -> rocket::Rocket {
    dotenv().ok();

    let host = env::var("WEB_API_HOST").expect("WEB_API_HOST is not set in .env file");
    let port = env::var("WEB_API_PORT").expect("WEB_API_PORT is not set in .env file");
    let origins = env::var("CORS_ORIGINS").expect("CORS_ORIGINS is not set in .env file");

    let mut config = Config::new(Environment::Development);
    config.set_address(&host).unwrap();
    config.set_port(u16::from_str(&port).unwrap_or_else(|_| 8888));

    let mut _origin = HashSet::new();
    let mut origin = Origins::default();
    _origin.insert(origins);
    origin.exact = Some(_origin);

    let cors_options = rocket_cors::CorsOptions::default()
        .max_age(Some(5 * 60))
        .allowed_origins(AllowedOrigins::Some(origin))
        .send_wildcard(false)
        .allow_credentials(true);

    let cors = rocket_cors::Cors::from_options(&cors_options).unwrap();

    rocket::custom(config)
        .mount("/user",
               routes![login, logout, authorized, not_authorized, register
                     , active, send_check_code, update_password])
        .mount("/console/task", routes![task_read, task_create, task_update, task_delete, task_execute, task_reload])
        .mount("/console/device", routes![device_read, device_create, device_update, device_delete])
        .attach(cors)
}

fn main() {
    dotenv().ok();

    let host = env::var("WEB_TO_CORE_HOST").expect("WEB_TO_CORE_HOST is not set in .env file");
    let port = env::var("WEB_TO_CORE_PORT").expect("WEB_TO_CORE_PORT is not set in .env file");
    let key = env::var("WEB_TO_CORE_KEY").expect("WEB_TO_CORE_KEY is not set in .env file");

    let core = TcpStream::connect(format!("{}:{}", host, port));
    let core = match core {
        Ok(mut s) => if let Ok(_) = s.write(key.as_bytes()) { Some(s) } else { None },
        Err(_) => None
    };
    rocket_web_api().manage(core).launch();
}
