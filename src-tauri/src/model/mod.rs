use async_trait::async_trait;

use crate::database::BaseModel;

pub(crate) mod service;


#[crud_table(table_name: project)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub path: Option<String>,
}

#[async_trait]
impl BaseModel<Project> for Project {
    fn meta(&mut self) -> Project {
        self.clone()
    }
}

#[crud_table(table_name: command)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: Option<i64>,
    pub project_id: Option<i64>,
    pub profile: Option<String>,
    pub remote_dir: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub target_name: Option<String>,
}

#[async_trait]
impl BaseModel<Command> for Command {
    fn meta(&mut self) -> Command {
        self.clone()
    }
}

#[crud_table(table_name: server)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i64>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub private_key: Option<String>,
    pub auth_type: Option<i64>,
}

#[async_trait]
impl BaseModel<Server> for Server {
    fn meta(&mut self) -> Server {
        self.clone()
    }
}

#[crud_table(table_name: datasource)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub db_type: Option<String>,
    pub host: Option<String>,
    pub port: Option<i64>,
    pub db_name: Option<String>,
    pub prefix: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[async_trait]
impl BaseModel<DataSource> for DataSource {
    fn meta(&mut self) -> DataSource {
        self.clone()
    }
}

#[crud_table(table_name: category)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i64>,
    pub parent_id: Option<i64>,
    pub name: Option<String>,
}

#[async_trait]
impl BaseModel<Category> for Category {
    fn meta(&mut self) -> Category {
        self.clone()
    }
}

#[crud_table(table_name: gen_project)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenProject {
    pub id: Option<i64>,
    pub project_id: Option<i64>,
    pub datasource: Option<i64>,
    pub output: Option<String>,
    pub template: Option<String>,
    pub type_mapping: Option<String>,
}

#[async_trait]
impl BaseModel<GenProject> for GenProject {
    fn meta(&mut self) -> GenProject {
        self.clone()
    }
}

#[crud_table(table_name: template)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: Option<i64>,
    pub category_id: Option<i64>,
    pub language: Option<String>,
    pub name: Option<String>,
    pub content: Option<String>,
}

#[async_trait]
impl BaseModel<Template> for Template {
    fn meta(&mut self) -> Template {
        self.clone()
    }
}

#[derive(CRUDTable, Debug, Clone, Deserialize, Serialize)]
pub struct Table {
    pub name: Option<String>,
    pub org_name: Option<String>,
    pub comment: Option<String>,
    pub column: Option<Vec<Column>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Column {
    pub name: Option<String>,
    pub field_name: Option<String>,
    pub data_type: Option<rbatis::core::types::byte::RbBytes>,
    pub field_type: Option<String>,
    pub key: Option<String>,
    pub comment: Option<String>,
}
#[derive(CRUDTable, Debug, Clone, Deserialize, Serialize)]
pub struct Env {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub value: Option<String>
}

#[async_trait]
impl BaseModel<Env> for Env {
    fn meta(&mut self) -> Env {
        self.clone()
    }
}