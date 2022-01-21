use std::path::Path;

use anyhow::{anyhow, Result};
use regex::Regex;
use tauri::Window;

use crate::cmd::CmdUtil;
use crate::cmd::ssh::SshUtil;
use crate::model::{Command, Server};
use crate::model::service::DeployInfo;

pub struct DeployUtil {
    pub cmd: CmdUtil,
    pub ssh: SshUtil,
    pub win: Window,
}


impl DeployUtil {
    pub fn new(win: Window, envs: Vec<String>) -> Result<DeployUtil> {
        let cmd = CmdUtil::new(win.clone(), envs);
        let ssh = SshUtil::new(win.clone());
        Ok(DeployUtil { cmd, win, ssh })
    }

    fn str_to_vec(value: &String) -> Vec<String> {
        let mut cmds = vec![];
        if !value.is_empty() {
            for str in value.split("\n") {
                cmds.push(str.to_string());
            }
        }
        cmds
    }

    fn replace_with_reg(reg: &Regex, value: &String, replace: &String) -> String {
        reg.replace_all(value, replace).to_string()
    }

    fn replace_cmd(cmds: Vec<String>, source_dir: &String, remote_dir: &String, target_name: &String) -> Vec<String> {
        let target_name_reg = Regex::new(r"(\{target_name\})").unwrap();
        let remote_dir_reg = Regex::new(r"(\{remote_dir\})").unwrap();
        let source_dir_reg = Regex::new(r"(\{source_dir\})").unwrap();

        cmds.iter().map(|x| x.as_str().to_string())
            .map(|x| DeployUtil::replace_with_reg(&target_name_reg, &x, target_name))
            .map(|x| DeployUtil::replace_with_reg(&remote_dir_reg, &x, remote_dir))
            .map(|x| DeployUtil::replace_with_reg(&source_dir_reg, &x, source_dir))
            .collect()
    }

    pub fn exec(&mut self, info: DeployInfo) -> Result<bool> {
        let command = info.profile;
        let source = info.project.path.unwrap();
        match self.before_deploy(&source, &command) {
            Ok(()) => {}
            Err(err) => {
                self.win.emit("console_error", err.to_string()).unwrap();
                return Err(anyhow!(err.to_string()));
            }
        }
        for server in info.servers {
            match self.deploy(&source, &server, &command) {
                Err(err) => {
                    self.win.emit("console", format!("服务器 {} 部署失败！({})", &server.name.unwrap(), err.to_string())).unwrap();
                    continue;
                }
                Ok(()) => {}
            }
        }
        Ok(true)
    }

    fn deploy(&mut self, source: &String, server: &Server, command: &Command) -> Result<()> {
        let name = server.name.as_ref().unwrap();
        let host = server.host.as_ref().unwrap();
        let port = server.port.as_ref().unwrap();
        let user = server.user.as_ref().unwrap();
        let password = server.password.as_ref().unwrap();
        let private_key = server.private_key.as_ref().unwrap();
        let auth_type = server.auth_type.as_ref().unwrap();
        let remote_dir = command.remote_dir.as_ref().unwrap();

        self.win.emit("console", format!("{} 部署开始！", name)).unwrap();
        match self.login_server(host, port, user, password, private_key, auth_type) {
            Err(err) => Err(anyhow!(err.to_string())),
            Ok(()) => {
                let file_path = Path::new(source).join(&command.target_name.as_ref().unwrap());
                let target_path = Path::new(remote_dir);
                self.ssh.check_dir(target_path)?;
                self.ssh.upload_file(file_path.as_path(), target_path.join(&command.target_name.as_ref().unwrap()).as_path())?;
                std::fs::remove_file(file_path)?;

                let after = DeployUtil::str_to_vec(&command.after.as_ref().unwrap());
                let after = DeployUtil::replace_cmd(after, source, remote_dir,command.target_name.as_ref().unwrap());
                for cmd in after {
                    self.ssh.exec(cmd)?;
                }
                self.win.emit("console", format!("{} 部署完成！", server.name.as_ref().unwrap())).unwrap();
                Ok(())
            }
        }
    }

    fn before_deploy(&mut self, source: &String, command: &Command) -> Result<()> {
        self.win.emit("console", "开始部署前置操作!").unwrap();
        let source_dir = source.clone();
        let target_file = Path::new(source).join(&command.target_name.as_ref().unwrap());
        if target_file.exists() {
            std::fs::remove_file(target_file)?;
        }
        self.cmd.change_path(source_dir);
        let before = DeployUtil::str_to_vec(&command.before.as_ref().unwrap());
        let before = DeployUtil::replace_cmd(before, source, command.remote_dir.as_ref().unwrap(),command.target_name.as_ref().unwrap());
        for cmd in before {
            self.cmd.exec(cmd)?;
        }
        self.win.emit("console", "完成部署前置操作!").unwrap();
        Ok(())
    }

    pub fn login_server(&mut self, host: &String, port: &i64, user: &String, password: &String, key_str: &String, auth_type: &i64) -> Result<()> {
        self.ssh.connect(host, port)?;
        if auth_type.eq(&1) {
            let private_key = Path::new(key_str);
            self.ssh.login_with_pubkey(user, private_key)?;
        } else {
            self.ssh.login_with_pwd(user, password)?;
        }
        Ok(())
    }
}