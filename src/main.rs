#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

mod user;
mod status;

use rocket::Config;
use rocket::config::Environment;
use crate::user::login::static_rocket_route_info_for_login;
use crate::user::register::static_rocket_route_info_for_register;

fn rocket_web_api() -> rocket::Rocket {
    let mut config = Config::new(Environment::Development);
    config.set_port(8080);
    rocket::custom(config)
        .mount("/user", routes![login, register])
}

fn main() {
    rocket_web_api().launch();
}
