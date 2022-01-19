use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use anyhow::{anyhow, Result};
use tauri::Window;

pub mod ssh;

fn status(code: i32) -> Result<()> {
    if code == 1 || code == 2 || code == 126 || code == 127 || code == 128 {
        Err(anyhow!(format!("命令执行错误！ {}", code)))
    } else {
        Ok(())
    }
}

#[derive(Clone)]
pub struct CmdUtil {
    pub current_dir: String,
    pub win: Window,
    pub envs: Vec<String>
}

impl CmdUtil {
    pub fn new(win: Window, envs: Vec<String>) -> CmdUtil {
        CmdUtil { current_dir: String::from(""), win , envs}
    }

    fn console(&self, msg: String) {
        self.win.emit("console", msg).unwrap();
    }

    pub fn change_path(&mut self, path: String) {
        self.current_dir = path;
    }

    pub fn exec(&self, cmd: String) -> Result<()> {
        self.console(format!("执行命令：{}", cmd));
        let mut out;
        if cfg!(target_os = "windows") {
            out = match self.current_dir.len() {
                0 => Command::new("powershell").stdin(Stdio::piped()).stdout(Stdio::piped()).arg(cmd).spawn()?,
                _ => {
                    Command::new("powershell").current_dir(&self.current_dir).stdin(Stdio::piped()).stdout(Stdio::piped()).arg(cmd).spawn()?
                }
            };
        } else {
            let env = match std::env::vars().find(|(k, _v)| k.eq("PATH")) {
                Some((_key, val))=> {
                    let mut envs = self.envs.clone();
                    envs.push(val);
                    envs.join(":")
                },
                None=> "".to_owned()
            };

            out = match self.current_dir.len() {
                0 => Command::new("sh").env("PATH", env).stdin(Stdio::piped()).stdout(Stdio::piped()).arg("-c").arg(cmd).spawn()?,
                _ => Command::new("sh").env("PATH", env).current_dir(&self.current_dir).stdin(Stdio::piped()).stdout(Stdio::piped()).arg("-c").arg(cmd).spawn()?
            };
        }
        let mut buf_reader = BufReader::new(out.stdout.take().unwrap());
        let mut line = String::new();
        let get_last_line = |lines: String| -> String {
            let array: Vec<&str> = lines.split("\n").collect();
            match array.len() {
                0 => lines,
                1 => array.get(0).unwrap().to_string(),
                2 => array.get(1).unwrap().to_string(),
                _ => array.get(array.len() - 2).unwrap().to_string()
            }
        };
        loop {
            match buf_reader.read_line(&mut line) {
                Ok(0) => break,
                _ => self.console(format!("{}", get_last_line(line.clone())))
            };
        };
        let status_code = out.wait().unwrap().code().unwrap();
        Ok(status(status_code)?)
    }
}