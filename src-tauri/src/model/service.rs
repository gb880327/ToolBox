use crate::model::{Command, Project, Server, Table};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeployInfo {
    pub project: Project,
    pub profile: Command,
    pub servers: Vec<Server>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub template_id: i64,
    pub file_path: String,
    pub file_name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenData {
    pub table: Vec<Table>,
    pub template: Vec<TemplateInfo>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenInfo {
    pub project_id: i64,
    pub tables: Vec<Table>,
    pub templates: Vec<TemplateInfo>
}