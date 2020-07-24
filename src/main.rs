#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod status;
mod user;
mod console;

use rocket::Config;
use rocket::config::Environment;

use crate::user::login::static_rocket_route_info_for_login;
use crate::user::auth::static_rocket_route_info_for_authorized;
use crate::user::auth::static_rocket_route_info_for_not_authorized;
use crate::user::register::static_rocket_route_info_for_register;

use crate::console::test::static_rocket_route_info_for_test;
use crate::console::test::static_rocket_route_info_for_test_error;

fn rocket_web_api() -> rocket::Rocket {
    let mut config = Config::new(Environment::Development);
    config.set_port(8080);
    rocket::custom(config)
        .mount("/user", routes![login, authorized, not_authorized, register])
        .mount("/console", routes![test, test_error])
}

fn main() {
    rocket_web_api().launch();
}
