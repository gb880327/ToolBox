use std::collections::HashMap;
use rbatis::{AsProxy, DriverType};
use rbatis::wrapper::Wrapper;
use serde_json::Value;
use tauri::async_runtime::block_on;

use crate::database::BaseModel;
use crate::model::*;

pub trait Param {
    fn get_int(&self, key: &str) -> i64;
}

impl Param for HashMap<String, Value> {
    fn get_int(&self, key: &str) -> i64 {
        self.get(key).expect("参数不存在!").i64()
    }
}

pub fn projects() -> Option<Vec<Project>> {
    block_on(Project::list(&super::RB, None))
}

pub fn deploy_projects(param: Option<HashMap<String, Value>>) -> Option<DeployProject> {
    if param.is_none() {
        return None;
    }
    let project_id = param.unwrap().get_int("projectId");
    block_on(DeployProject::one(&super::RB, Wrapper::new(&DriverType::Sqlite).eq("project_id", project_id)))
}