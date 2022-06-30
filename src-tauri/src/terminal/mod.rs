use std::borrow::Cow;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{anyhow, Result};
use dialoguer::{Confirm, MultiSelect, Select};
use dialoguer::console::{Style, style};
use dialoguer::theme::ColorfulTheme;
use regex::Regex;
use ssh2::FileStat;
use crate::app::Asset;
use crate::cmd::ssh::SshUtil;
use crate::database::{BaseModel, new_wrapper};
use crate::model::{Command, Project, Server};
use crate::model::service::DeployInfo;
use crate::service::Service;

fn colorful_theme() -> ColorfulTheme {
    ColorfulTheme {
        defaults_style: Style::new().for_stderr().cyan(),
        prompt_style: Style::new().for_stderr().bold(),
        prompt_prefix: style("?".to_string()).for_stderr().yellow(),
        prompt_suffix: style("›".to_string()).for_stderr().black().bright(),
        success_prefix: style("✔".to_string()).for_stderr().blue(),
        success_suffix: style("·".to_string()).for_stderr().black().bright(),
        error_prefix: style("✘".to_string()).for_stderr().red(),
        error_style: Style::new().for_stderr().red(),
        hint_style: Style::new().for_stderr().black().bright(),
        values_style: Style::new().for_stderr().blue(),
        active_item_style: Style::new().for_stderr().cyan(),
        inactive_item_style: Style::new().for_stderr(),
        active_item_prefix: style("❯".to_string()).for_stderr().blue(),
        inactive_item_prefix: style(" ".to_string()).for_stderr(),
        checked_item_prefix: style("✔".to_string()).for_stderr().blue(),
        unchecked_item_prefix: style(" ".to_string()).for_stderr().black(),
        picked_item_prefix: style("❯".to_string()).for_stderr().blue(),
        unpicked_item_prefix: style(" ".to_string()).for_stderr(),
        inline_selections: true,
    }
}

fn exec_ssh(server: &Server) -> Result<()> {
    match Asset::get("login.sh") {
        Some(content) => {
            let mut ssh_script = match content.data {
                Cow::Borrowed(bytes) => String::from_utf8(bytes.into()).expect("ssh登陆脚本读取错误！"),
                Cow::Owned(bytes) => String::from_utf8(bytes.into()).expect("ssh登陆脚本读取错误！"),
            };
            let reg = Regex::new("[\\[|\\]\\\\|\"|'|$|&|*|?|~|`|!|#|\\||{|}|;|<|>|^]").expect("正则表达式错误！");
            if server.auth_type == Some(0 as i64) {
                let pwd = server.password.as_ref().unwrap();
                let pwd = reg.replace_all(pwd, "\\$0");
                ssh_script = ssh_script.replace("{ssh}", &*format!("ssh -p {} {}@{}",
                                                                   server.port.as_ref().unwrap(),
                                                                   server.user.as_ref().unwrap(),
                                                                   server.host.as_ref().unwrap()))
                    .replace("{pwd}", &pwd);
            } else {
                ssh_script = ssh_script.replace("{ssh}", &*format!("ssh -i {} -p {} {}@{}",
                                                                   server.private_key.as_ref().cloned().unwrap(),
                                                                   server.port.as_ref().unwrap(),
                                                                   server.user.as_ref().unwrap(),
                                                                   server.host.as_ref().unwrap()))
                    .replace("{pwd}", "");
            }
            match server.command.as_ref() {
                Some(command) => {
                    if !command.is_empty() {
                        let cmd = reg.replace_all(command, "\\$0");
                        ssh_script = ssh_script.replace("{cmd}", &*format!("send \"{}\\r\"", cmd));
                    } else {
                        ssh_script = ssh_script.replace("{cmd}", "");
                    }
                }
                None => {
                    ssh_script = ssh_script.replace("{cmd}", "");
                }
            }
            let mut spawn = process::Command::new("sh").arg("-c").arg(ssh_script).spawn()?;
            spawn.wait().expect("运行ssh脚本错误！");
            Ok(())
        }
        None => Err(anyhow!("ssh登陆脚本不存在！"))
    }
}

pub async fn ssh_login(server: Option<&str>) -> Result<()> {
    match server {
        Some(ser) => {
            match super::SERVICE.lock().unwrap().find_server(ser).await {
                Some(select_server) => exec_ssh(&select_server),
                None => Err(anyhow!("服务器不存在！"))
            }
        }
        None => {
            let server = select_server().await?;
            exec_ssh(&server)
        }
    }
}

pub async fn list_server() -> Result<()> {
    match super::SERVICE.lock().unwrap().server_list().await {
        Some(servers) => {
            for server in servers {
                println!("{} - {} - {}", server.name.as_ref().unwrap(), server.label.as_ref().unwrap_or(&String::new()), server.host.as_ref().unwrap());
            }
            Ok(())
        }
        None => Err(anyhow!("无服务器信息！"))
    }
}

async fn select_project() -> Result<Project> {
    match super::SERVICE.lock().unwrap().project_list().await {
        Some(projects) => {
            let items: Vec<String> = projects.iter().map(|x| x.name.as_ref().unwrap().clone()).collect();
            let select = Select::with_theme(&colorful_theme()).items(&items).default(0).with_prompt("请选择项目(默认选择第一个)").interact()?;
            Ok(projects.get(select).unwrap().clone())
        }
        None => Err(anyhow!("请添加项目信息！"))
    }
}

async fn select_server() -> Result<Server> {
    match super::SERVICE.lock().unwrap().server_list().await {
        Some(servers) => {
            let items = get_label(&servers);
            let select = Select::with_theme(&colorful_theme()).items(&items).default(0).with_prompt("请选择服务器(默认选择第一个)").interact()?;
            Ok(servers.get(select).unwrap().clone())
        }
        None => Err(anyhow!("请添加服务器！"))
    }
}

async fn select_servers() -> Result<Vec<Server>> {
    match super::SERVICE.lock().unwrap().server_list().await {
        Some(servers) => {
            let items = get_label(&servers);
            let mut select: Vec<usize> = MultiSelect::with_theme(&colorful_theme()).items(&items).with_prompt("请选择目标服务器").interact()?;
            while select.is_empty() {
                select = MultiSelect::with_theme(&colorful_theme()).items(&items).with_prompt("请选择目标服务器").interact()?;
            }
            let mut select_server: Vec<Server> = Vec::new();
            for i in select {
                select_server.push(servers.get(i).unwrap().clone());
            }
            Ok(select_server)
        }
        None => Err(anyhow!("请添加服务器信息！"))
    }
}

fn get_label(servers: &Vec<Server>) -> Vec<String> {
    let items: Vec<String> = servers.iter().map(|x| format!("{} - {} - {}", x.name.as_ref().unwrap().clone(), x.label.as_ref().unwrap_or(&String::new()).clone(), x.host.as_ref().unwrap().clone())).collect();
    items
}

async fn select_profile(project_id: i64) -> Result<Command> {
    match super::SERVICE.lock().unwrap().project_profiles(project_id).await {
        Some(commands) => {
            if commands.len() == 1 {
                Ok(commands.get(0).unwrap().clone())
            } else {
                let items: Vec<String> = commands.iter().map(|x| x.profile.as_ref().unwrap().clone()).collect();
                let select = Select::with_theme(&colorful_theme()).items(&items).default(0).with_prompt("请选择部署环境(默认选择第一个)").interact()?;
                Ok(commands.get(select).unwrap().clone())
            }
        }
        None => Err(anyhow!("请添加部署配置！"))
    }
}

pub async fn deploy() -> Result<()> {
    let project = select_project().await?;
    let servers = select_servers().await?;
    let profile = select_profile(project.id.unwrap()).await?;
    let deploy_info = DeployInfo { project, servers, profile };

    let envs = super::SERVICE.lock().unwrap().env_list().await;
    match envs {
        Some(data) => {
            let env_list: Vec<String> = data.iter().map(|x| x.value.as_ref().unwrap().to_owned()).collect();
            Service::deploy_project_comm(deploy_info, env_list)?;
        }
        None => Service::deploy_project_comm(deploy_info, Vec::new())?
    }
    Ok(())
}

fn download_file(ssh: &mut SshUtil, remote: &Path, local: &Path) -> Result<()> {
    if ssh.check_is_dir(remote)? {
        let files: Vec<(PathBuf, FileStat)> = ssh.file_list(remote)?;
        let current = local.join(remote.file_name().unwrap());
        if !current.exists() {
            create_dir(current.as_path())?;
        }
        for file in files {
            download_file(ssh, &file.0, current.as_path())?;
        }
    } else {
        if local.is_file() {
            ssh.download_sftp(local, remote)?;
        } else {
            ssh.download_sftp(&*local.join(remote.file_name().unwrap()), remote)?;
        }
    }
    Ok(())
}

fn upload_file(ssh: &mut SshUtil, remote: &Path, local: &Path) -> Result<()> {
    if local.is_dir() {
        let files = local.read_dir()?;
        let file_name = local.file_name().unwrap();
        let current = remote.join(file_name);
        ssh.check_remote_dir(current.as_path(), true)?;
        for file in files {
            let fs = file?;
            upload_file(ssh, current.as_path(), &fs.path())?
        }
    } else {
        if remote.is_file() {
            ssh.upload_file(local, remote)?;
        } else {
            ssh.upload_file(local, &*remote.join(local.file_name().unwrap()))?;
        }
    }
    Ok(())
}

pub async fn sftp_file(file: &str, path: &str) -> Result<()> {
    let (oper, server_name, local, remote) = parse_path(file, path)?;
    let local_path = check_path(local);
    let server: Result<Server> = match super::SERVICE.lock().unwrap().find_server(&server_name).await {
        Some(data) => Ok(data),
        None => Err(anyhow!("服务器不存在!"))
    };
    let server = server?;
    let remote_path = check_remote_path(remote, server.user.as_ref().unwrap().clone());
    let mut ssh = SshUtil::new();
    ssh.connect(server.host.as_ref().unwrap(), server.port.as_ref().unwrap())?;
    if server.auth_type.as_ref().unwrap().eq(&1) {
        let private_key = Path::new(server.private_key.as_ref().unwrap());
        ssh.login_with_pubkey(server.user.as_ref().unwrap(), private_key)?;
    } else {
        ssh.login_with_pwd(server.user.as_ref().unwrap(), server.password.as_ref().unwrap())?;
    }
    if oper == 0 {
        if !local_path.exists() {
            return Err(anyhow!("文件不存在！"));
        } else {
            println!("开始上传");
            upload_file(&mut ssh, &remote_path, &local_path)?;
            println!("上传结束");
        }
    } else {
        println!("开始下载");
        ssh.check_remote_dir(remote_path.as_path(), false)?;
        download_file(&mut ssh, &remote_path, &local_path)?;
        println!("下载结束");
    }
    Ok(())
}

fn parse_path(file: &str, path: &str) -> Result<(i32, String, String, String)> {
    if file.contains(":") && !path.contains(":") {
        // 下载文件
        let temp: (&str, &str) = split_path(file)?;
        Ok((1, temp.0.to_string(), path.to_string(), temp.1.to_string()))
    } else if !file.contains(":") && path.contains(":") {
        // 上传文件
        let temp: (&str, &str) = split_path(path)?;
        Ok((0, temp.0.to_string(), file.to_string(), temp.1.to_string()))
    } else {
        Err(anyhow!("参数格式错误！"))
    }
}

fn split_path(path: &str) -> Result<(&str, &str)> {
    let temp = path.split_once(":").unwrap();
    if temp.0.is_empty() || temp.1.is_empty() {
        return Err(anyhow!("参数格式错误！"));
    }
    Ok(temp)
}

fn check_remote_path(path: String, user: String) -> PathBuf {
    if path.starts_with("./") || path.starts_with("~/") {
        return if user.eq("root") {
            PathBuf::from("/root").join(path.replace("./", "").replace("~/", ""))
        } else {
            PathBuf::from(format!("/home/{}", user)).join(path.replace("./", ""))
        };
    } else {
        PathBuf::from(path)
    }
}

fn check_path(path: String) -> PathBuf {
    if path.starts_with("./") {
        std::env::current_dir().unwrap().join(path.replace("./", ""))
    } else {
        PathBuf::from(path)
    }
}

pub async fn remove_server(server_name: &str) -> Result<()> {
    if Confirm::new().with_prompt("确定要删除此服务器配置？").interact()? {
        match super::SERVICE.lock().unwrap().find_server(server_name).await {
            Some(server) => {
                match Server::remove(&super::RB, Some(new_wrapper().eq("id", server.id.as_ref().unwrap()))).await {
                    Some(i) => {
                        if i > 0 {
                            println!("服务器 {} 已删除！", server_name);
                            Ok(())
                        } else {
                            Err(anyhow!("服务器配置删除失败！"))
                        }
                    }
                    None => Err(anyhow!("服务器配置删除失败！"))
                }
            }
            None => Err(anyhow!("服务器配置不存在！"))
        }
    } else {
        Ok(())
    }
}