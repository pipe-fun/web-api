use chrono::{NaiveDateTime, NaiveTime};
use status_protoc::status::db_api::{DbAPIStatus, _DbAPIStatus};
use std::collections::HashMap;
use rocket::State;
use std::net::TcpStream;
use rocket_contrib::json::Json;
use status_protoc::status::console::task::{TaskStatus, _TaskStatus};
use status_protoc::my_trait::StatusTrait;
use web2core::protoc::{OpResult, ExecuteInfo, Operation};
use std::io::{Write, Read};
use crate::console::device;
use crate::console::device::Device;
use crate::user::auth::APIToken;
use crate::user::tools::check_response;

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
pub fn task_read(token: APIToken) -> Json<Vec<Task>> {
    let tasks = match read() {
        Ok(ts) => ts,
        Err(_) => Vec::new()
    };

    let tasks = tasks.into_iter()
        .filter(|t| t.owner.eq(&token.0))
        .collect::<Vec<Task>>();

    Json(tasks)
}

#[delete("/delete/<id>")]
pub fn task_delete(_token: APIToken, id: i32) -> Json<TaskStatus> {
    let status = match delete(id) {
        Ok(()) => TaskStatus::default(),
        Err(e) => TaskStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[post("/create", format = "json", data = "<info>")]
pub fn task_create(token: APIToken, mut info: Json<NewTask>) -> Json<TaskStatus> {
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
pub fn task_update(_token: APIToken, id: i32, info: Json<NewTask>) -> Json<TaskStatus> {
    let status = match update(&info.into_inner(), id) {
        Ok(()) => TaskStatus::default(),
        Err(e) => TaskStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[get("/reload/<token>")]
pub fn task_reload(_token: APIToken, token: String, core: State<Option<TcpStream>>)
                   -> Json<OpResult> {
    if core.inner().is_none() {
        return Json(OpResult::CoreOffline);
    }

    let mut core = core.inner().as_ref().unwrap();
    let buf = serde_json::to_string(&Operation::Reload(token.clone())).unwrap();

    let result = match core.write(buf.as_bytes()) {
        Ok(_) => {
            let mut buf = [0; 1024];
            let len = core.read(&mut buf).unwrap();
            let result = core::str::from_utf8(&buf[0..len]).unwrap();
            serde_json::from_str(&result).unwrap()
        }
        Err(_) => OpResult::CoreOffline,
    };

    Json(result)
}

#[post("/execute", format = "json", data = "<info>")]
pub fn task_execute(_token: APIToken, info: Json<Task>, core: State<Option<TcpStream>>)
                    -> Json<OpResult> {
    if core.inner().is_none() {
        return Json(OpResult::CoreOffline);
    }

    let mut core = core.inner().as_ref().unwrap();
    let info = ExecuteInfo::new(&info.device_token, &info.command);
    let buf = serde_json::to_string(&Operation::Execute(info)).unwrap();

    let result = match core.write(buf.as_bytes()) {
        Ok(_) => {
            let mut buf = [0; 1024];
            let len = core.read(&mut buf).unwrap();
            let result = core::str::from_utf8(&buf[0..len]).unwrap();
            serde_json::from_str(&result).unwrap()
        }
        Err(_) => OpResult::CoreOffline,
    };

    Json(result)
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
