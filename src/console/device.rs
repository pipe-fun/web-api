use status_protoc::status::db_api::DbAPIStatus;
use rocket_contrib::json::Json;
use status_protoc::status::console::device::DeviceStatus;
use status_protoc::my_trait::StatusTrait;
use uuid::Uuid;
use crate::user::auth::APIToken;
use crate::request;

#[derive(Serialize, Deserialize)]
pub struct NewDevice {
    name: String,
    owner: String
}

impl NewDevice {
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
pub fn device_read(token: APIToken) -> Json<Vec<Device>> {
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
pub fn device_delete(_token: APIToken, token: String) -> Json<DeviceStatus> {
    let status = match delete(&token) {
        Ok(()) => DeviceStatus::default(),
        Err(e) => DeviceStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[post("/create", format = "json", data = "<info>")]
pub fn device_create(token: APIToken, info: Json<NewDevice>) -> Json<DeviceStatus> {
    let mut device = info.into_inner().to_device();
    device.owner = token.0;
    let status = match create(&device) {
        Ok(()) => DeviceStatus::default(),
        Err(e) => DeviceStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

#[put("/update", format = "json", data = "<info>")]
pub fn device_update(_token: APIToken, info: Json<Device>) -> Json<DeviceStatus> {
    let status = match update(&info.into_inner()) {
        Ok(()) => DeviceStatus::default(),
        Err(e) => DeviceStatus::set_db_api_err_simple(e)
    };

    Json(status)
}

pub fn create(info: &Device) -> Result<(), DbAPIStatus> {
    request::post("http://localhost:1122/device/create", info)
}

pub fn read() -> Result<Vec<Device>, DbAPIStatus> {
    request::get_all("http://localhost:1122/device/read")
}

pub fn delete(token: &str) -> Result<(), DbAPIStatus> {
    let url = format!("http://localhost:1122/device/delete/{}", token);
    request::delete(&url)
}

pub fn update(device: &Device) -> Result<(), DbAPIStatus> {
    let url = format!("http://localhost:1122/device/update/{}", device.token);
    request::put(&url, device)
}
