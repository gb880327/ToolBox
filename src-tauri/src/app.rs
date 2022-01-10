use std::borrow::Cow;
use std::ops::Add;
use std::sync::mpsc;

use anyhow::{anyhow, Result};
use dirs::config_dir;
use rbatis::rbatis::Rbatis;
use rust_embed::RustEmbed;
use serde_json::{Map, Value};
use tauri::Window;

use crate::database;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

#[derive(Serialize, Debug)]
pub struct ResponseMsg<T> {
    method: MethodEvent,
    code: i64,
    data: Option<T>,
    msg: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestParam {
    pub method: MethodEvent,
    pub param: Option<Map<String, Value>>,
}

struct ConfigInfo {
    pub path: String,
    pub is_new: bool,
}

fn get_db_path() -> Result<ConfigInfo> {
    let mut config_path = config_dir().unwrap().join("ToolBox");
    if !config_path.exists() {
        std::fs::create_dir(config_path.clone()).expect("配置文件初始化失败！");
    }
    config_path = if cfg!(debug_assertions) {
        config_path.join("config_test.db")
    } else {
        config_path.join("config.db")
    };
    let is_new = !config_path.exists();
    if is_new {
        std::fs::File::create(config_path.clone()).expect("配置文件初始化失败！");
    }
    let db_path = config_path.to_str().expect("配置文件路径不存在！");
    Ok(ConfigInfo { path: db_path.to_string(), is_new })
}

pub async fn init(rb: &Rbatis) -> Result<()> {
    let config = get_db_path()?;
    let db_url = String::from("sqlite://").add(config.path.as_str());
    rb.link(&db_url).await?;
    if config.is_new {
        match Asset::get("table.sql") {
            Some(content) => {
                let sql = match content.data {
                    Cow::Borrowed(bytes) => String::from_utf8(bytes.into()).expect("sql文件读取错误！"),
                    Cow::Owned(bytes) => String::from_utf8(bytes.into()).expect("sql文件读取错误！"),
                };
                database::exec_sql(rb, sql.as_str(), None).await.expect("数据库初始化失败！");
            }
            None => {}
        }
    }
    Ok(())
}

pub fn unwrap<T: serde::Serialize + core::fmt::Debug>(method: MethodEvent, data: Option<T>) -> Result<String> {
    let resp: ResponseMsg<T> = ResponseMsg { method, code: 200, data, msg: None };
    match serde_json::to_string(&resp) {
        Ok(json) => Ok(json),
        Err(err) => Err(anyhow!(err.to_string()))
    }
}

#[tauri::command]
pub fn exec(win: Window, params: RequestParam) {
    let (tx, rx) = mpsc::channel();
    let thread = std::thread::spawn(move || tx.send(exec_method(params)).unwrap());
    match thread.join() {
        Ok(_t) => {}
        Err(_err) => win.emit("error", "接口调用错误！").unwrap()
    };
    let resp = rx.recv().unwrap();
    match resp {
        Ok(t) => win.emit("response", t).unwrap(),
        Err(err) => win.emit("error", err.to_string()).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MethodEvent {
    ProjectList,
    SaveProject,
    RemoveProject,
    Servers,
    SaveServer,
    RemoveServer,
    DeploySetting,
    SaveDeploySetting,
    Deploy,
    DataSources,
    SaveDataSource,
    RemoveDataSource,
    Categorys,
    SaveCategory,
    RemoveCategory,
    Templates,
    SaveTemplate,
    RemoveTemplate,
    GenSetting,
    SaveGenSetting,
    TableAndTemplate,
    GenTemplate,
}

fn exec_method(params: RequestParam) -> Result<String> {
    super::SERVICE.lock().unwrap().set_param(params.param);
    match params.method {
        MethodEvent::ProjectList => unwrap(params.method, super::SERVICE.lock().unwrap().projects()),
        MethodEvent::SaveProject => unwrap(params.method, super::SERVICE.lock().unwrap().save_project()),
        MethodEvent::RemoveProject => unwrap(params.method, super::SERVICE.lock().unwrap().remove_project()),
        MethodEvent::Servers => unwrap(params.method, super::SERVICE.lock().unwrap().servers()),
        MethodEvent::SaveServer => unwrap(params.method, super::SERVICE.lock().unwrap().save_server()),
        MethodEvent::RemoveServer => unwrap(params.method, super::SERVICE.lock().unwrap().remove_server()),
        MethodEvent::DeploySetting => unwrap(params.method, super::SERVICE.lock().unwrap().deploy_setting()),
        MethodEvent::SaveDeploySetting => unwrap(params.method, super::SERVICE.lock().unwrap().save_deploy_setting()),
        MethodEvent::Deploy => unwrap(params.method, super::SERVICE.lock().unwrap().deploy_project()),
        MethodEvent::DataSources => unwrap(params.method, super::SERVICE.lock().unwrap().datasources()),
        MethodEvent::SaveDataSource => unwrap(params.method, super::SERVICE.lock().unwrap().save_datasource()),
        MethodEvent::RemoveDataSource => unwrap(params.method, super::SERVICE.lock().unwrap().remove_datasource()),
        MethodEvent::Categorys => unwrap(params.method, super::SERVICE.lock().unwrap().categorys()),
        MethodEvent::SaveCategory => unwrap(params.method, super::SERVICE.lock().unwrap().save_category()),
        MethodEvent::RemoveCategory => unwrap(params.method, super::SERVICE.lock().unwrap().remove_category()),
        MethodEvent::Templates => unwrap(params.method, super::SERVICE.lock().unwrap().templates()),
        MethodEvent::SaveTemplate => unwrap(params.method, super::SERVICE.lock().unwrap().save_template()),
        MethodEvent::RemoveTemplate => unwrap(params.method, super::SERVICE.lock().unwrap().remove_template()),
        MethodEvent::GenSetting => unwrap(params.method, super::SERVICE.lock().unwrap().gen_setting()),
        MethodEvent::SaveGenSetting => unwrap(params.method, super::SERVICE.lock().unwrap().save_gen_setting()),
        MethodEvent::TableAndTemplate => unwrap(params.method, super::SERVICE.lock().unwrap().table_and_template()),
        MethodEvent::GenTemplate => unwrap(params.method, super::SERVICE.lock().unwrap().gen_template())
    }
}