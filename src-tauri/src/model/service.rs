use crate::model::{Command, Project, Server};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeployInfo {
    pub project: Project,
    pub profile: Command,
    pub servers: Vec<Server>,
}