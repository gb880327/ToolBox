use std::ops::Add;

use rbatis::{AsProxy, Error};
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use serde::Deserialize;
use serde_json::{Map, Value};
use tauri::async_runtime::block_on;
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
    pub param: Option<Map<String, Value>>,
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
        let data: Page<Project> = block_on(super::RB.fetch_page_by_wrapper(new_wrapper(), &page_request)).unwrap();
        Some(data)
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

        let envs = match block_on(Env::list(&super::RB, None)){
            Some(val)=> {
                let tmp: Vec<String> = val.iter().map(|x| x.value.as_ref().unwrap().to_owned()).collect();
                tmp
            },
            None=> vec![]
        };

        std::thread::spawn(move || {
            let mut deploy_util = DeployUtil::new(win.clone(), envs).unwrap();
            match deploy_util.exec(deploy_info) {
                Ok(_rst) => {},
                Err(err) => {
                    win.emit("error", err.to_string()).unwrap();
                }
            }
            win.emit("console", "over").unwrap()
        });
        Some(true)
    }

    pub fn datasources(&mut self) -> Option<Page<DataSource>> {
        let page_request = self.param.as_ref()?.get_page_param()?;
        block_on(DataSource::list_by_page(&super::RB, None, page_request))
    }

    pub fn save_datasource(&mut self) -> Option<bool> {
        let ds: DataSource = self.param.as_ref()?.get_bean("ds")?;
        Some(block_on(save_or_update(&super::RB, &ds, ds.id))? > 0)
    }

    pub fn remove_datasource(&mut self) -> Option<bool> {
        let ds_id = self.param.as_ref()?.get_int("id")?;
        Some(block_on(DataSource::remove(&super::RB, Some(new_wrapper().eq("id", ds_id))))? > 0)
    }

    pub fn categorys(&mut self) -> Option<Vec<Category>> {
        Some(block_on(Category::list(&super::RB, None))?)
    }

    pub fn save_category(&mut self) -> Option<bool> {
        let category: Category = self.param.as_ref()?.get_bean("category")?;
        Some(block_on(save_or_update(&super::RB, &category, category.id))? > 0)
    }

    pub fn remove_category(&mut self) -> Option<bool> {
        let id = self.param.as_ref()?.get_int("id")?;
        Some(block_on(Category::remove(&super::RB, Some(new_wrapper().eq("id", id))))? > 0)
    }

    pub fn templates(&mut self) -> Option<Page<Template>> {
        let category = self.param.as_ref()?.get_int("categoryId");
        let page_request = self.param.as_ref()?.get_page_param()?;
        let wrapper = match category {
            Some(id) => Some(new_wrapper().eq("category_id", id)),
            None => None
        };
        block_on(Template::list_by_page(&super::RB, wrapper, page_request))
    }

    pub fn save_template(&mut self) -> Option<bool> {
        let template: Template = self.param.as_ref()?.get_bean("template")?;
        Some(block_on(save_or_update(&super::RB, &template, template.id))? > 0)
    }

    pub fn remove_template(&mut self) -> Option<bool> {
        let id = self.param.as_ref()?.get_int("templateId")?;
        Some(block_on(Template::remove(&super::RB, Some(new_wrapper().eq("id", id))))? > 0)
    }

    pub fn gen_setting(&mut self) -> Option<GenProject> {
        let project_id = self.param.as_ref()?.get_int("projectId")?;
        let gen_project: Option<GenProject> = block_on(GenProject::one(&super::RB, new_wrapper().eq("project_id", project_id)));
        gen_project
    }

    pub fn save_gen_setting(&mut self) -> Option<bool> {
        let setting: GenProject = self.param.as_ref()?.get_bean("setting")?;
        Some(block_on(save_or_update(&super::RB, &setting, setting.id))? > 0)
    }

    pub fn table_and_template(&mut self) -> Option<GenData> {
        let project_id = self.param.as_ref()?.get_int("projectId")?;
        let gen_project: GenProject = block_on(GenProject::one(&super::RB, new_wrapper().eq("project_id", project_id)))?;
        let template_info: Vec<TemplateInfo> = match serde_json::from_str(&gen_project.template?) {
            Ok(result) => result,
            Err(_err) => vec![]
        };
        let db_id = gen_project.datasource.clone()?;
        let db_info: DataSource = block_on(DataSource::one(&super::RB, new_wrapper().eq("id", db_id)))?;
        block_on(super::MYSQL.link(&format!("mysql://{}:{}@{}:{}/{}", db_info.user?, db_info.password?, db_info.host?, db_info.port?, db_info.db_name.clone()?))).unwrap();
        let table: Result<Vec<Table>, Error> = block_on(table_list(&db_info.db_name.clone()?));
        match table {
            Ok(t) => Some(GenData { table: t, template: template_info }),
            Err(_err) => Some(GenData { table: vec![], template: template_info })
        }
    }

    pub fn gen_template(&mut self) -> Option<bool> {
        let gen_info: GenInfo = self.param.as_ref()?.get_bean("genInfo")?;
        let gen_project: GenProject = block_on(GenProject::one(&super::RB, new_wrapper().eq("project_id", gen_info.project_id)))?;
        let project: Project = block_on(Project::one(&super::RB, new_wrapper().eq("id", gen_info.project_id)))?;

        let db_id = gen_project.datasource.clone()?;
        let db_info: DataSource = block_on(DataSource::one(&super::RB, new_wrapper().eq("id", db_id)))?;
        let prefix = db_info.prefix?;
        let db_name = db_info.db_name?;

        let mut tables = gen_info.tables;
        for table in tables.iter_mut() {
            table.name = Some(get_table_name(table.org_name.as_ref()?.clone(), prefix.clone()));

            let column: Result<Vec<Column>, Error> = block_on(table_column(&db_name, &table.org_name.as_ref()?));
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
                    println!("== {}", err.to_string());
                    table.column = None
                }
            }
        }
        let templates = gen_info.templates;
        let mut template_params: Vec<TemplateParam> = vec![];
        for tpl in templates.iter() {
            let tp: Template = block_on(Template::one(&super::RB, new_wrapper().eq("id", tpl.template_id)))?;
            template_params.push(TemplateParam {
                file_path: tpl.file_path.clone(),
                file_name: tpl.file_name.clone(),
                content: tp.content?,
                lang: tp.language?,
            });
        }
        let mut template_render = TemplateRender { table: tables, root_path: project.path?, output: gen_project.output?, templates: template_params };
        match template_render.render(self.win.clone()?) {
            Ok(()) => Some(true),
            Err(_err) => Some(false)
        }
    }

    pub fn envs(&mut self)-> Option<Page<Env>> {
        block_on(Env::list_by_page(&super::RB, None, self.param.as_ref()?.get_page_param()?))
    }

    pub fn save_env(&mut self)->Option<bool> {
        let env: Env = self.param.as_ref()?.get_bean("env")?;
        Some(block_on(save_or_update(&super::RB, &env, env.id))? > 0)
    }

    pub fn remove_env(&mut self) -> Option<bool> {
        let id = self.param.as_ref()?.get_int("id")?;
        Some(block_on(Env::remove(&super::RB, Some(new_wrapper().eq("id", id))))? > 0)
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