use std::ops::Add;

use anyhow::Result;
use rbatis::{AsProxy, Error};
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use serde::Deserialize;
use serde_json::{Map, Value};
use tauri::Window;

use crate::database::{BaseModel, new_wrapper, save_or_update, table_column, table_list};
use crate::deploy::DeployUtil;
use crate::gencode::{TemplateParam, TemplateRender};
use crate::model::*;
use crate::model::service::{DeployInfo, GenData, GenInfo, TemplateInfo};

pub trait Param {
    fn get_int(&self, key: &str) -> Option<i64>;
    fn get_str(&self, key: &str) -> Option<String>;
    fn get_bool(&self, key: &str) -> Option<bool>;
    fn get_page_param(&self) -> Option<PageRequest>;
    fn get_bean<T>(&self, key: &str) -> Option<T>
        where
            T: for<'de> Deserialize<'de> + std::fmt::Debug;
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
        let page_no = if self.contains_key("pageNum") {
            self.get_int("pageNum")?
        } else {
            1
        };
        let page_size = if self.contains_key("pageSize") {
            self.get_int("pageSize")?
        } else {
            1000
        };
        Some(PageRequest::new(page_no as u64, page_size as u64))
    }

    fn get_bean<T>(&self, key: &str) -> Option<T>
        where
            T: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        let val = self.get(key)?.clone();
        let t: T = serde_json::value::from_value(val).unwrap();
        Some(t)
    }
}

pub struct Service {
    pub win: Option<Window>,
    pub param: Option<Map<String, Value>>,
    pub is_win: bool,
}

impl Service {
    pub fn default() -> Service {
        Service {
            win: None,
            param: None,
            is_win: false,
        }
    }

    pub fn set_win(&mut self, win: Window) {
        self.win = Some(win);
        self.is_win = true
    }

    pub fn console(&mut self, msg_type: &str, msg: String) {
        if msg.is_empty() {
            return;
        }
        match self.win.as_ref() {
            Some(win) => {
                win.emit(msg_type, msg).unwrap();
            }
            None => {
                println!("{}", msg);
            }
        }
    }

    pub fn set_param(&mut self, param: Option<Map<String, Value>>) {
        self.param = param
    }

    pub async fn projects(&mut self) -> Option<Page<Project>> {
        let page_request = self.param.as_ref().unwrap().get_page_param().unwrap();
        Project::list_by_page(&super::RB, None, page_request).await
    }

    pub async fn project_list(&mut self) -> Option<Vec<Project>> {
        Project::list(&super::RB, None).await
    }

    pub async fn save_project(&mut self) -> Option<bool> {
        let project: Project = self.param.as_ref()?.get_bean("project")?;
        Some(save_or_update(&super::RB, &project, project.id).await? > 0)
    }

    pub async fn remove_project(&mut self) -> Option<bool> {
        let project_id = self.param.as_ref()?.get_int("id")?;
        Some(Project::remove(&super::RB, Some(new_wrapper().eq("id", project_id))).await? > 0)
    }

    pub async fn servers(&mut self) -> Option<Page<Server>> {
        Server::list_by_page(&super::RB, None, self.param.as_ref()?.get_page_param()?).await
    }

    pub async fn server_list(&mut self) -> Option<Vec<Server>> {
        Server::list(&super::RB, None).await
    }

    pub async fn find_server(&mut self, server: &str) -> Option<Server> {
        Server::one(&super::RB, new_wrapper().eq("name", server)).await
    }

    pub async fn save_server(&mut self) -> Option<bool> {
        let server: Server = self.param.as_ref()?.get_bean("server")?;
        Some(save_or_update(&super::RB, &server, server.id).await? > 0)
    }

    pub async fn remove_server(&mut self) -> Option<bool> {
        let id = self.param.as_ref()?.get_int("id")?;
        Some(Server::remove(&super::RB, Some(new_wrapper().eq("id", id))).await? > 0)
    }

    pub async fn project_profiles(&mut self, project_id: i64) -> Option<Vec<Command>> {
        Command::list(&super::RB, Some(new_wrapper().eq("project_id", project_id))).await
    }

    pub async fn deploy_setting(&mut self) -> Option<Vec<Command>> {
        let project_id = self.param.as_ref()?.get_int("id")?;
        Command::list(&super::RB, Some(new_wrapper().eq("project_id", project_id))).await
    }

    pub async fn save_deploy_setting(&mut self) -> Option<bool> {
        let p = self.param.as_ref()?;
        let project_id = p.get_int("project_id")?;
        let setting: Vec<Command> = p.get_bean("command")?;
        match super::RB.remove_by_column::<Command, _>("project_id", &project_id).await {
            Ok(_rst) => match super::RB.save_batch(&setting, &[]).await {
                Ok(rt) => Some(rt.rows_affected > 0),
                Err(err) => {
                    self.console("error", err.to_string());
                    Some(false)
                }
            },
            Err(err) => {
                self.console("error", err.to_string());
                Some(false)
            }
        }
    }

    pub async fn deploy_project(&mut self) -> Option<bool> {
        let deploy_info: DeployInfo = self.param.as_ref()?.get_bean("info")?;
        let envs = match Env::list(&super::RB, None).await {
            Some(val) => {
                let tmp: Vec<String> = val
                    .iter()
                    .map(|x| x.value.as_ref().unwrap().to_owned())
                    .collect();
                tmp
            }
            None => vec![],
        };
        std::thread::spawn(move || {
            Service::deploy_project_comm(deploy_info, envs).unwrap();
        });
        Some(true)
    }

    pub fn deploy_project_comm(deploy_info: DeployInfo, envs: Vec<String>) -> Result<()> {
        let mut deploy_util = DeployUtil::new(envs)?;
        match deploy_util.exec(deploy_info) {
            Ok(_rst) => {}
            Err(err) => {
                super::SERVICE.lock().unwrap().console("error", err.to_string());
            }
        }
        super::SERVICE.lock().unwrap().console("console", "over".to_string());
        Ok(())
    }

    pub async fn datasources(&mut self) -> Option<Page<DataSource>> {
        let page_request = self.param.as_ref()?.get_page_param()?;
        DataSource::list_by_page(&super::RB, None, page_request).await
    }

    pub async fn save_datasource(&mut self) -> Option<bool> {
        let ds: DataSource = self.param.as_ref()?.get_bean("ds")?;
        Some(save_or_update(&super::RB, &ds, ds.id).await? > 0)
    }

    pub async fn remove_datasource(&mut self) -> Option<bool> {
        let ds_id = self.param.as_ref()?.get_int("id")?;
        Some(DataSource::remove(&super::RB, Some(new_wrapper().eq("id", ds_id))).await? > 0)
    }

    pub async fn categorys(&mut self) -> Option<Vec<Category>> {
        Category::list(&super::RB, None).await
    }

    pub async fn save_category(&mut self) -> Option<bool> {
        let category: Category = self.param.as_ref()?.get_bean("category")?;
        Some(save_or_update(&super::RB, &category, category.id).await? > 0)
    }

    pub async fn remove_category(&mut self) -> Option<bool> {
        let id = self.param.as_ref()?.get_int("id")?;
        Some(Category::remove(&super::RB, Some(new_wrapper().eq("id", id))).await? > 0)
    }

    pub async fn templates(&mut self) -> Option<Page<Template>> {
        let category = self.param.as_ref()?.get_int("categoryId");
        let page_request = self.param.as_ref()?.get_page_param()?;
        let wrapper = match category {
            Some(id) => Some(new_wrapper().eq("category_id", id)),
            None => None,
        };
        Template::list_by_page(&super::RB, wrapper, page_request).await
    }

    pub async fn save_template(&mut self) -> Option<bool> {
        let template: Template = self.param.as_ref()?.get_bean("template")?;
        Some(save_or_update(&super::RB, &template, template.id).await? > 0)
    }

    pub async fn remove_template(&mut self) -> Option<bool> {
        let id = self.param.as_ref()?.get_int("templateId")?;
        Some(Template::remove(&super::RB, Some(new_wrapper().eq("id", id))).await? > 0)
    }

    pub async fn gen_setting(&mut self) -> Option<GenProject> {
        let project_id = self.param.as_ref()?.get_int("projectId")?;
        GenProject::one(&super::RB, new_wrapper().eq("project_id", project_id)).await
    }

    pub async fn save_gen_setting(&mut self) -> Option<bool> {
        let setting: GenProject = self.param.as_ref()?.get_bean("setting")?;
        Some(save_or_update(&super::RB, &setting, setting.id).await? > 0)
    }

    pub async fn table_and_template(&mut self) -> Option<GenData> {
        let project_id = self.param.as_ref()?.get_int("projectId")?;
        let gen_project: GenProject = GenProject::one(&super::RB, new_wrapper().eq("project_id", project_id)).await?;
        let template_info: Vec<TemplateInfo> = match serde_json::from_str(&gen_project.template?) {
            Ok(result) => result,
            Err(_err) => vec![],
        };
        let db_id = gen_project.datasource.clone()?;
        let db_info: DataSource = DataSource::one(&super::RB, new_wrapper().eq("id", db_id)).await?;
        super::MYSQL.link(&format!("mysql://{}:{}@{}:{}/{}",
                                   db_info.user?,
                                   db_info.password?,
                                   db_info.host?,
                                   db_info.port?,
                                   db_info.db_name.clone()?
        )).await.unwrap();
        let table: Result<Vec<Table>, Error> = table_list(db_info.db_name.as_ref()?).await;
        match table {
            Ok(t) => Some(GenData {
                table: t,
                template: template_info,
            }),
            Err(_err) => Some(GenData {
                table: vec![],
                template: template_info,
            }),
        }
    }

    pub async fn gen_template(&mut self) -> Option<bool> {
        let gen_info: GenInfo = self.param.as_ref()?.get_bean("genInfo")?;
        let gen_project: GenProject = GenProject::one(&super::RB, new_wrapper().eq("project_id", gen_info.project_id)).await?;
        let project: Project = Project::one(&super::RB, new_wrapper().eq("id", gen_info.project_id)).await?;

        let db_id = gen_project.datasource.clone()?;
        let db_info: DataSource = DataSource::one(&super::RB, new_wrapper().eq("id", db_id)).await?;
        let prefix = db_info.prefix?;
        let db_name = db_info.db_name?;

        let mut tables = gen_info.tables;
        for table in tables.iter_mut() {
            table.name = Some(get_table_name(
                table.org_name.as_ref()?.clone(),
                prefix.clone(),
            ));

            let column: Result<Vec<Column>, Error> = table_column(&db_name, table.org_name.as_ref()?).await;
            match column {
                Ok(mut cols) => {
                    for col in cols.iter_mut() {
                        let field_type = col.data_type.as_ref()?.clone();
                        col.field_type = Some(String::from_utf8(field_type.rb_bytes).unwrap());
                        col.name = Some(get_column_name(col.field_name.as_ref()?.clone()));
                    }
                    table.column = Some(cols)
                }
                Err(err) => {
                    self.console("error", err.to_string());
                    table.column = None
                }
            }
        }
        let templates = gen_info.templates;
        let mut template_params: Vec<TemplateParam> = vec![];
        for tpl in templates.iter() {
            let tp: Template = Template::one(&super::RB, new_wrapper().eq("id", tpl.template_id)).await?;
            template_params.push(TemplateParam {
                file_path: tpl.file_path.clone(),
                file_name: tpl.file_name.clone(),
                content: tp.content?,
                lang: tp.language?,
            });
        }
        let mut template_render = TemplateRender {
            table: tables,
            root_path: project.path?,
            output: gen_project.output?,
            templates: template_params,
        };
        match template_render.render() {
            Ok(()) => Some(true),
            Err(_err) => Some(false),
        }
    }

    pub async fn envs(&mut self) -> Option<Page<Env>> {
        Env::list_by_page(&super::RB, None, self.param.as_ref()?.get_page_param()?).await
    }

    pub async fn env_list(&mut self) -> Option<Vec<Env>> {
        Env::list(&super::RB, None).await
    }

    pub async fn save_env(&mut self) -> Option<bool> {
        let env: Env = self.param.as_ref()?.get_bean("env")?;
        Some(save_or_update(&super::RB, &env, env.id).await? > 0)
    }

    pub async fn remove_env(&mut self) -> Option<bool> {
        let id = self.param.as_ref()?.get_int("id")?;
        Some(Env::remove(&super::RB, Some(new_wrapper().eq("id", id))).await? > 0)
    }
}

/**
 * 处理表名 驼峰命名规则
 */
fn get_table_name(name: String, prefix: String) -> String {
    let mut table_name = name;
    if !prefix.is_empty() {
        table_name = table_name.replace(&prefix, "");
    }
    let strs = table_name.split("_");
    let mut new_table_name = String::new();
    for item in strs {
        new_table_name = new_table_name.add(&*item.get_name())
    }
    new_table_name
}

fn get_column_name(col: String) -> String {
    let col = col.replace(" ", "");
    let strs: Vec<&str> = col.split("_").collect();
    let mut new_name = String::new();
    let mut index = 0;
    for s in strs {
        if index == 0 {
            new_name = new_name.add(&s);
        } else {
            new_name = new_name.add(&*s.get_name());
        }
        index = index + 1;
    }
    new_name
}

trait TableName {
    fn get_name(&self) -> String;
}

impl TableName for &str {
    fn get_name(&self) -> String {
        if !self.is_ascii() || self.is_empty() {
            return String::from(*self);
        }
        let (head, tail) = self.split_at(1);
        head.to_uppercase() + tail
    }
}
