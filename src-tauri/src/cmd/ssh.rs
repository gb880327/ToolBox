use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

use anyhow::{anyhow, Result};
use ssh2::Session;
use tauri::Window;
use super::status;

#[derive(Clone)]
pub struct SshUtil {
    pub session: Option<Session>,
    pub win: Window,
}

impl SshUtil {
    pub fn new(win: Window) -> SshUtil {
        SshUtil { session: None, win }
    }

    pub fn connect(&mut self, host: String, port: i64) -> Result<()> {
        let mut session = Session::new()?;
        let mut server = String::from(host);
        server.push(':');
        server.push_str(&port.to_string());
        match TcpStream::connect(server) {
            Ok(tcp) => {
                session.set_tcp_stream(tcp);
                session.set_compress(true);
                session.set_timeout(30000);
                session.handshake()?;
                self.session = Some(session);
                Ok(())
            }
            Err(err) => Err(anyhow!(err.to_string()))
        }
    }

    fn console(&self, msg: String) {
        self.win.emit("console", msg).unwrap();
    }

    pub fn login_with_pwd(&mut self, name: String, password: String) -> Result<()> {
        Ok(self.session.as_ref().unwrap().userauth_password(&name, &password)?)
    }

    pub fn login_with_pubkey(&mut self, name: String, private_key: &Path) -> Result<()> {
        Ok(self.session.as_ref().unwrap().userauth_pubkey_file(&name, None, private_key, None)?)
    }

    pub fn exec(&mut self, cmd: String) -> Result<()> {
        self.console(format!("执行命令：{}", cmd));
        let mut channel = self.session.as_ref().unwrap().channel_session()?;
        channel.exec(&cmd)?;
        let mut result = String::new();
        channel.read_to_string(&mut result)?;
        self.console(format!("{}", result));
        result.clear();
        channel.stderr().read_to_string(&mut result)?;
        self.console(format!("{}", result));
        channel.send_eof()?;
        channel.wait_eof()?;
        channel.wait_close()?;

        let status_code = channel.exit_status()?;
        Ok(status(status_code)?)
    }

    pub fn upload_file(&mut self, file_path: &Path, remote_path: &Path) -> Result<()> {
        self.console("开始文件上传！".into());
        let mut fs = File::open(file_path)?;
        let len = fs.metadata()?.len();
        let remote_file = self.session.as_ref().unwrap().scp_send(remote_path, 0o644, len.clone(), None);
        match remote_file {
            Err(e) => Err(anyhow!(e.to_string())),
            _ => {
                let mut buf = vec![0; (match len <= 1000 {
                    true => len,
                    false => len / 1000
                }) as usize];
                let mut remote_file = remote_file?;

                let mut pos = 0;
                while pos < len {
                    pos = pos + fs.read(&mut buf.as_mut_slice())? as u64;
                    remote_file.write_all(&buf.as_slice())?;
                    self.win.emit("console_progress", (pos as f64 * 100.0) / len as f64).unwrap();
                }
                self.console("文件上传完成!".into());
                remote_file.send_eof()?;
                remote_file.wait_eof()?;
                remote_file.close()?;
                remote_file.wait_close()?;
                Ok(())
            }
        }
    }

    // pub fn download_sftp(&mut self, file_path: &Path, remote_path: &Path) -> Result<()> {
    //     self.console("开始下载文件！".into());
    //     let sftp = self.session.as_ref().unwrap().sftp()?;
    //     let capacity = 1024 * 1024;
    //     let fs = sftp.open(remote_path)?;

    //     // let len = fs.stat().unwrap().size.unwrap();
    //     let mut input_file = std::io::BufReader::with_capacity(capacity, fs);
    //     let mut fs = OpenOptions::new().create(true).append(true).open(file_path)?;
    //     std::io::copy(&mut input_file, &mut fs)?;
    //     Ok(())
    // }

    // pub fn is_dir(&mut self, path: &Path) -> Result<bool> {
    //     match self.session.as_ref().unwrap().sftp() {
    //         Ok(sftp) => {
    //             let stat = sftp.stat(path)?;
    //             Ok(stat.is_dir())
    //         }
    //         Err(err) => Err(anyhow!(err.to_string()))
    //     }
    // }

    pub fn check_dir(&mut self, path: &Path) -> Result<()> {
        match self.session.as_ref().unwrap().sftp() {
            Ok(sftp) => {
                match sftp.stat(path) {
                    Err(_e) => {
                        Ok(sftp.mkdir(path, 0o644)?)
                    }
                    Ok(_stat) => Ok(())
                }
            }
            Err(err) => Err(anyhow!(err.to_string()))
        }
    }
}