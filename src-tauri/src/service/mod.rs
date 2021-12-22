use std::collections::HashMap;

use rbatis::{AsProxy};
use rbatis::plugin::page::{Page, PageRequest};
use serde::Deserialize;
use serde_json::Value;
use tauri::async_runtime::block_on;

use crate::database::{BaseModel, new_wrapper};
use crate::model::*;

pub trait Param {
    fn get_int(&self, key: &str) -> Option<i64>;
    fn get_page_param(&self) -> Option<PageRequest>;
    fn get_bean<T>(&self) -> Option<T> where T: for<'de> Deserialize<'de> + std::fmt::Debug;
}

impl Param for HashMap<String, Value> {
    fn get_int(&self, key: &str) -> Option<i64> {
        if self.contains_key(key) { Some(self.get(key)?.i64()) } else { None }
    }

    fn get_page_param(&self) -> Option<PageRequest> {
        let page_no = if self.contains_key("pageNum") { self.get_int("pageNum")? } else { 1 };
        let page_size = if self.contains_key("pageSize") { self.get_int("pageSize")? } else { 10 };
        Some(PageRequest::new(page_no as u64, page_size as u64))
    }

    fn get_bean<T>(&self) -> Option<T> where T: for<'de> Deserialize<'de> + std::fmt::Debug {
        let json = serde_json::to_string(self).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}


pub fn projects(param: Option<HashMap<String, Value>>) -> Option<Page<Project>> {
    block_on(Project::list_by_page(&super::RB, None, param?.get_page_param()?))
}

pub fn save_project(param: Option<HashMap<String, Value>>) -> Option<bool> {
    let mut  project: Project = param?.get_bean()?;
    let mut count = 0;
    if project.id > 0 {
        count = block_on(project.update(&super::RB, None))?
    } else {
        count = block_on(project.save(&super::RB))?
    }
    Some(count > 0)
}

pub fn remove_project(param: Option<HashMap<String, Value>>) -> Option<bool> {
    let project_id = param?.get_int("id")?;
    let count = block_on(Project::remove(&super::RB, Some(new_wrapper().eq("id", project_id))))?;
    Some(count > 0)
}

pub fn deploy_projects(param: Option<HashMap<String, Value>>) -> Option<DeployProject> {
    let project_id = param?.get_int("projectId");
    block_on(DeployProject::one(&super::RB, new_wrapper().eq("project_id", project_id)))
}