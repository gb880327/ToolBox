use async_trait::async_trait;

use crate::database::BaseModel;

#[crud_table(table_name: project)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: String
}

#[async_trait]
impl BaseModel<Project> for Project {
    fn meta(&self) -> Project {
        self.clone()
    }
}

#[crud_table(table_name: deploy_project)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeployProject {
    pub id: i64,
    pub project_id: i64,
    pub source_dir: String,
    pub target_name: String,
}

#[async_trait]
impl BaseModel<DeployProject> for DeployProject {
    fn meta(&self) -> DeployProject {
        self.clone()
    }
}

#[crud_table(table_name: command)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: i64,
    pub project_id: i64,
    pub profile: String,
    pub remote_dir: String,
    pub before: String,
    pub after: String,
}

#[async_trait]
impl BaseModel<Command> for Command {
    fn meta(&self) -> Command {
        self.clone()
    }
}

#[crud_table(table_name: server)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub host: String,
    pub port: i64,
    pub user: String,
    pub password: String,
    pub private_key: String,
}

#[async_trait]
impl BaseModel<Server> for Server {
    fn meta(&self) -> Server {
        self.clone()
    }
}

#[crud_table(table_name: datasource)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub id: i64,
    pub name: String,
    pub db_type: String,
    pub host: String,
    pub port: i64,
    pub db_name: String,
    pub prefix: String,
    pub user: String,
    pub password: String,
}

#[async_trait]
impl BaseModel<DataSource> for DataSource {
    fn meta(&self) -> DataSource {
        self.clone()
    }
}

#[crud_table(table_name: category)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[async_trait]
impl BaseModel<Category> for Category {
    fn meta(&self) -> Category {
        self.clone()
    }
}

#[crud_table(table_name: gen_project)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenProject {
    pub id: i64,
    pub project_id: i64,
    pub datasource: i64,
    pub root_path: String,
    pub output: String,
    pub template: String,
    pub type_mapping: String,
}

#[async_trait]
impl BaseModel<GenProject> for GenProject {
    fn meta(&self) -> GenProject {
        self.clone()
    }
}

#[crud_table(table_name: template)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: i64,
    pub category_id: i64,
    pub language: String,
    pub name: String,
    pub content: String,
}

#[async_trait]
impl BaseModel<Template> for Template {
    fn meta(&self) -> Template {
        self.clone()
    }
}