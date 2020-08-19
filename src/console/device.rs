use status_protoc::status::db_api::{DbAPIStatus, _DbAPIStatus};
use std::collections::HashMap;
use crate::user::tools::check_response;
use crate::user::auth::ApiToken;
use rocket_contrib::json::Json;
use status_protoc::status::console::device::DeviceStatus;
use status_protoc::my_trait::StatusTrait;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PreDevice {
    name: String,
    owner: String
}

impl PreDevice {
    fn to_device(self) -> Device {
        let uuid = Uuid::new_v4();
        Device {
            token: uuid.to_string(),
            name: self.name,
            owner: self.owner
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub token: String,
    name: String,
    owner: String
}

#[get("/read")]
pub fn device_read(token: ApiToken) -> Json<Vec<Device>> {
    let devices = match read() {
        Ok(ts) => ts,
        Err(_) => Vec::new()
    };

    let devices = devices.into_iter()
        .filter(|d| d.owner.eq(&token.0))
        .collect::<Vec<Device>>();

    Json(devices)
}

#[delete("/delete/<token>")]
pub fn device_delete(_token: ApiToken, token: String) -> Json<DeviceStatus> {
    let status = match delete(&token) {
        Ok(()) => DeviceStatus::default(),
        Err(e) => DeviceStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[post("/create", format = "json", data = "<info>")]
pub fn device_create(_token: ApiToken, info: Json<PreDevice>) -> Json<DeviceStatus> {
    let device = info.into_inner().to_device();
    let status = match create(&device) {
        Ok(()) => DeviceStatus::default(),
        Err(e) => DeviceStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[put("/update", format = "json", data = "<info>")]
pub fn device_update(_token: ApiToken, info: Json<Device>) -> Json<DeviceStatus> {
    let status = match update(&info.into_inner()) {
        Ok(()) => DeviceStatus::default(),
        Err(e) => DeviceStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

pub fn create(info: &Device) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.post("http://localhost:1122/api/device/create").json(&info).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
    }
}

pub fn read() -> Result<Vec<Device>, DbAPIStatus> {
    match reqwest::blocking::get("http://localhost:1122/api/device/read") {
        Ok(response) => {
            match response.json::<Vec<Device>>() {
                Ok(device) => Ok(device),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn delete(token: &str) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/device/delete/{}", token);
    match client.delete(&uri).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}


pub fn update(device: &Device) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/device/update/{}", device.token);
    match client.put(&uri).json(device).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}
