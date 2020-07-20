#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod schema;
mod user;
mod db;

use rocket::Config;
use rocket::config::Environment;

use crate::db::user::static_rocket_route_info_for_read;
use crate::db::user::static_rocket_route_info_for_create;
use crate::db::user::static_rocket_route_info_for_update;
use crate::db::user::static_rocket_route_info_for_delete;
use crate::user::login::static_rocket_route_info_for_login;

#[database("info")]
pub struct DbConn(diesel::MysqlConnection);

fn rocket_db() -> rocket::Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/db/user", routes![read, create, update, delete])
}

fn rocket_web_api() -> rocket::Rocket {
    let mut config = Config::new(Environment::Development);
    config.set_port(8080);
    rocket::custom(config)
        .mount("/user", routes![login])
}

fn main() {
    rocket_web_api().launch();
}
