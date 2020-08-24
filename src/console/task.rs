use chrono::{NaiveDateTime, NaiveTime};
use status_protoc::status::db_api::{DbAPIStatus, _DbAPIStatus};
use std::collections::HashMap;
use crate::user::tools::check_response;
use rocket_contrib::json::Json;
use status_protoc::status::console::task::{TaskStatus, _TaskStatus};
use status_protoc::my_trait::StatusTrait;
use crate::console::device;
use crate::console::device::Device;
use crate::user::auth::ApiToken;

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: i32,
    name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    owner: String,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewTask {
    name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    owner: String,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

#[get("/read")]
pub fn task_read(token: ApiToken) -> Json<Vec<Task>> {
    let tasks = match read() {
        Ok(ts) => ts,
        Err(_) => Vec::new()
    };

    let tasks = tasks.into_iter()
        .filter(|t| t.owner.eq(&token.0))
        .collect::<Vec<Task>>();

    Json(tasks)
}

#[get("/read_by_id/<id>")]
pub fn task_read_by_id(_token: ApiToken, id: i32) -> Json<Task> {
    let tasks = match read() {
        Ok(ts) => ts,
        Err(_) => Vec::new()
    };

    let mut tasks = tasks.into_iter()
        .filter(|t| t.id == id)
        .collect::<Vec<Task>>();

    let task = tasks.pop().unwrap();
    Json(task)
}

#[delete("/delete/<id>")]
pub fn task_delete(_token: ApiToken, id: i32) -> Json<TaskStatus> {
    let status = match delete(id) {
        Ok(()) => TaskStatus::default(),
        Err(e) => TaskStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[post("/create", format = "json", data = "<info>")]
pub fn task_create(token: ApiToken, mut info: Json<NewTask>) -> Json<TaskStatus> {
    let pre_status = match device::read() {
        Ok(ds) => {
            let ds = ds.into_iter()
                .filter(|d| d.token.eq(&info.device_token))
                .collect::<Vec<Device>>();
            if ds.is_empty() {
                TaskStatus::default().set_status(_TaskStatus::TokenHasNoExist)
            } else {
                TaskStatus::default()
            }
        }
        Err(e) => TaskStatus::set_db_api_err_simple(e)
    };

    if pre_status.status_code() != 0 { return Json(pre_status); }

    info.owner = token.0;
    let status = match create(&info.into_inner()) {
        Ok(()) => TaskStatus::default(),
        Err(e) => TaskStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[put("/update/<id>", format = "json", data = "<info>")]
pub fn task_update(_token: ApiToken, id: i32, info: Json<NewTask>) -> Json<TaskStatus> {
    let status = match update(&info.into_inner(), id) {
        Ok(()) => TaskStatus::default(),
        Err(e) => TaskStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

pub fn create(info: &NewTask) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.post("http://localhost:1122/api/task/create").json(&info).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
    }
}

pub fn read() -> Result<Vec<Task>, DbAPIStatus> {
    match reqwest::blocking::get("http://localhost:1122/api/task/read") {
        Ok(response) => {
            match response.json::<Vec<Task>>() {
                Ok(task) => Ok(task),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn delete(id: i32) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/task/delete/{}", id);
    match client.delete(&uri).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string())),
    }
}

pub fn update(task: &NewTask, id: i32) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/task/update/{}", id);
    match client.put(&uri).json(task).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}
