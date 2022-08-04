#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate serde;

use std::sync::Mutex;

use clap::{App, Arg, SubCommand};
use rbatis::rbatis::Rbatis;
use tauri::{LogicalPosition, LogicalSize, Menu, MenuItem, Position, Size, Submenu};

use app::exec;
use service::Service;

mod model;
mod database;
mod app;
mod gencode;
mod cmd;
mod deploy;
mod service;
mod terminal;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref SERVICE: Mutex<Service> = Mutex::new(Service::default());
}


#[tauri::command]
fn exit(window: tauri::Window) {
    window.close().unwrap()
}

#[tokio::main]
async fn main() {
    app::init(&RB).await.unwrap();

    let server_arg = SubCommand::with_name("server").about("服务器管理")
        .subcommand(SubCommand::with_name("list").about("查询服务器列表"))
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("serverName")).about("删除服务器"));

    let ssh_arg = SubCommand::with_name("ssh").about("登陆服务器")
        .arg(Arg::with_name("server"));

    let scp_arg = SubCommand::with_name("scp").arg(Arg::with_name("file")).arg(Arg::with_name("path")).about("文件上传与下载");

    let matchs = App::new("ToolBox").version("1.0")
        .author("Rookie. <gb880327@189.cn>")
        .about("Rookie的工具箱")
        .subcommand(server_arg)
        .subcommand(SubCommand::with_name("deploy").about("部署应用").arg(Arg::with_name("manual").short('m').takes_value(false)))
        .subcommand(ssh_arg)
        .subcommand(SubCommand::with_name("mgr").about("管理界面"))
        .subcommand(scp_arg)
        .get_matches();

    async fn ssh_login(server: Option<&str>) {
        match terminal::ssh_login(server).await {
            Ok(()) => {}
            Err(err) => SERVICE.lock().unwrap().console("error", err.to_string())
        }
    }

    fn show_ui() {
        let menu = Menu::new().add_submenu(Submenu::new("编辑", Menu::new()
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::SelectAll)
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Quit)));
        tauri::Builder::default()
            .on_page_load(move |win, _| {
                win.set_title("Rookie的工具箱").unwrap();
                let position = Position::Logical(LogicalPosition { x: 100 as f64, y: 100 as f64 });
                win.set_position(position).unwrap();
                let size = Size::Logical(LogicalSize { width: 1440 as f64, height: 800 as f64 });
                win.set_size(size).unwrap();
                SERVICE.lock().unwrap().set_win(win.clone());
            })
            .menu(menu)
            .invoke_handler(tauri::generate_handler![exec, exit])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    match matchs.subcommand_name() {
        Some(sub) => {
            match sub {
                "deploy" => {
                    let deploy = matchs.subcommand_matches("deploy").expect("命令错误！");
                    if deploy.contains_id("manual") {
                        match terminal::deploy().await {
                            Ok(()) => {}
                            Err(err) => SERVICE.lock().unwrap().console("error", err.to_string())
                        }
                    } else {
                        match terminal::quick_deploy().await {
                            Ok(()) => {}
                            Err(err) => SERVICE.lock().unwrap().console("error", err.to_string())
                        }
                    }
                }
                "ssh" => {
                    let ssh = matchs.subcommand_matches("ssh").expect("参数错误！");
                    ssh_login(ssh.value_of("server")).await;
                }
                "mgr" => {
                    show_ui();
                }
                "scp" => {
                    let scp = matchs.subcommand_matches("scp").expect("参数错误！");
                    let file = scp.value_of("file").expect("参数错误！");
                    let path = scp.value_of("path").expect("参数错误！");
                    match terminal::sftp_file(file, path).await {
                        Ok(()) => {}
                        Err(err) => println!("{}", err)
                    }
                }
                "server" => {
                    let server = matchs.subcommand_matches("server").expect("参数错误！");
                    match server.subcommand_name() {
                        Some(sub_cmd) => {
                            match sub_cmd {
                                "list" => terminal::list_server().await.unwrap(),
                                "rm" => {
                                    let server_name = server.subcommand_matches("rm").expect("参数错误！").value_of("serverName").expect("参数错误！");
                                    terminal::remove_server(server_name).await.expect("删除服务器失败！");
                                }
                                _ => println!("参数错误！请查看帮助文档. rt --help")
                            }
                        }
                        None => terminal::list_server().await.unwrap()
                    }
                }
                _ => {
                    println!("参数错误！请查看帮助文档. rt --help");
                }
            }
        }
        None => {
            if cfg!(debug_assertions) {
                show_ui()
            } else {
                ssh_login(None).await
            }
        }
    };
}
