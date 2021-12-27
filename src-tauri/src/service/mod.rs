use rbatis::AsProxy;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use serde::Deserialize;
use serde_json::{Map, Value};
use tauri::async_runtime::block_on;
use tauri::Window;

use crate::database::{BaseModel, new_wrapper, save_or_update};
use crate::deploy::DeployUtil;
use crate::model::*;
use crate::model::service::DeployInfo;

pub trait Param {
    fn get_int(&self, key: &str) -> Option<i64>;
    fn get_str(&self, key: &str) -> Option<String>;
    fn get_bool(&self, key: &str) -> Option<bool>;
    fn get_page_param(&self) -> Option<PageRequest>;
    fn get_bean<T>(&self, key: &str) -> Option<T> where T: for<'de> Deserialize<'de> + std::fmt::Debug;
}

impl Param for Map<String, Value> {
    fn get_int(&self, key: &str) -> Option<i64> {
        Some(self.get(key)?.i64())
    }

    fn get_str(&self, key: &str) -> Option<String> {
        Some(self.get(key)?.string())
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        Some(self.get(key)?.bool())
    }

    fn get_page_param(&self) -> Option<PageRequest> {
        let page_no = if self.contains_key("pageNum") { self.get_int("pageNum")? } else { 1 };
        let page_size = if self.contains_key("pageSize") { self.get_int("pageSize")? } else { 10 };
        Some(PageRequest::new(page_no as u64, page_size as u64))
    }

    fn get_bean<T>(&self, key: &str) -> Option<T> where T: for<'de> Deserialize<'de> + std::fmt::Debug {
        let val = self.get(key)?.clone();
        let t: T = serde_json::value::from_value(val).unwrap();
        Some(t)
    }
}

pub struct Service {
    pub win: Option<Window>,
    pub param: Option<Map<String, Value>>
}

impl Service {
    pub fn default() -> Service {
        Service { win: None, param: None }
    }

    pub fn set_win(&mut self, win: Window) {
        self.win = Some(win)
    }

    pub fn set_param(&mut self, param: Option<Map<String, Value>>) {
        self.param = param
    }

    pub fn projects(&mut self) -> Option<Page<Project>> {
        let page_request = self.param.as_ref()?.get_page_param()?;
        block_on(Project::list_by_page(&super::RB, None, page_request))
    }

    pub fn save_project(&mut self) -> Option<bool> {
        let project: Project = self.param.as_ref()?.get_bean("project")?;
        Some(block_on(save_or_update(&super::RB, &project, project.id))? > 0)
    }

    pub fn remove_project(&mut self) -> Option<bool> {
        let project_id = self.param.as_ref()?.get_int("id")?;
        Some(block_on(Project::remove(&super::RB, Some(new_wrapper().eq("id", project_id))))? > 0)
    }

    pub fn servers(&mut self) -> Option<Page<Server>> {
        block_on(Server::list_by_page(&super::RB, None, self.param.as_ref()?.get_page_param()?))
    }

    pub fn save_server(&mut self) -> Option<bool> {
        let server: Server = self.param.as_ref()?.get_bean("server")?;
        Some(block_on(save_or_update(&super::RB, &server, server.id))? > 0)
    }

    pub fn remove_server(&mut self) -> Option<bool> {
        let id = self.param.as_ref()?.get_int("id")?;
        Some(block_on(Server::remove(&super::RB, Some(new_wrapper().eq("id", id))))? > 0)
    }

    pub fn deploy_setting(&mut self) -> Option<Vec<Command>> {
        let project_id = self.param.as_ref()?.get_int("id")?;
        block_on(Command::list(&super::RB, Some(new_wrapper().eq("project_id", project_id))))
    }

    pub fn save_deploy_setting(&mut self) -> Option<bool> {
        let p = self.param.as_ref()?;
        let project_id = p.get_int("project_id")?;
        let setting: Vec<Command> = p.get_bean("command")?;
        match block_on(super::RB.remove_by_column::<Command, _>("project_id", &project_id)) {
            Ok(_rst) => {
                match block_on(super::RB.save_batch(&setting, &[])) {
                    Ok(rt) => {
                        Some(rt.rows_affected > 0)
                    }
                    Err(err) => {
                        self.win.as_ref()?.emit("error", err.to_string()).unwrap();
                        Some(false)
                    }
                }
            }
            Err(err) => {
                self.win.as_ref()?.emit("error", err.to_string()).unwrap();
                Some(false)
            }
        }
    }

    pub fn deploy_project(&mut self) -> Option<bool> {
        let deploy_info: DeployInfo = self.param.as_ref()?.get_bean("info")?;
        let win = self.win.as_mut()?.clone();
        std::thread::spawn(move || {
            let mut deploy_util = DeployUtil::new(win.clone()).unwrap();
            match deploy_util.exec(deploy_info) {
                Ok(_rst) => win.emit("console", "over").unwrap(),
                Err(err) => {
                    win.emit("error", err.to_string()).unwrap();
                }
            }
        });
        Some(true)
    }
}