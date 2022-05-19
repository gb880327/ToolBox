use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use indicatif::{ProgressBar, ProgressStyle};
use ssh2::{FileStat, Session};

use super::status;

#[derive(Clone)]
pub struct SshUtil {
    pub session: Option<Session>,
}

impl SshUtil {
    pub fn new() -> SshUtil {
        SshUtil { session: None }
    }

    pub fn connect(&mut self, host: &String, port: &i64) -> Result<()> {
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
        super::super::SERVICE.lock().unwrap().console("console", msg);
    }

    pub fn login_with_pwd(&mut self, name: &String, password: &String) -> Result<()> {
        Ok(self.session.as_ref().unwrap().userauth_password(name, password)?)
    }

    pub fn login_with_pubkey(&mut self, name: &String, private_key: &Path) -> Result<()> {
        Ok(self.session.as_ref().unwrap().userauth_pubkey_file(name, None, private_key, None)?)
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
        // Ok(status(status_code)?)
        Ok(())
    }

    pub fn upload_file(&mut self, file_path: &Path, remote_path: &Path) -> Result<()> {
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
                let is_win = super::super::SERVICE.lock().unwrap().is_win;
                if !is_win {
                    let pb = ProgressBar::new(len.clone());
                    let file_name = file_path.file_name().unwrap().to_str().unwrap();
                    pb.set_style(ProgressStyle::default_bar()
                        .template(&*format!("{0} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] {{bytes}}/{{total_bytes}} ({{bytes_per_sec}}, {{eta}})", file_name))
                        .progress_chars("#>-"));
                    while pos < len {
                        pos = pos + fs.read(&mut buf.as_mut_slice())? as u64;
                        remote_file.write_all(&buf.as_slice())?;
                        pb.set_position(pos);
                    }
                    pb.finish();
                } else {
                    while pos < len {
                        pos = pos + fs.read(&mut buf.as_mut_slice())? as u64;
                        remote_file.write_all(&buf.as_slice())?;
                        let tmp = (pos as f64 * 100.0) / len as f64;
                        super::super::SERVICE.lock().unwrap().console("console_progress", tmp.to_string());
                    }
                }
                remote_file.send_eof()?;
                remote_file.wait_eof()?;
                remote_file.close()?;
                remote_file.wait_close()?;
                Ok(())
            }
        }
    }

    pub fn download_sftp(&mut self, file_path: &Path, remote_path: &Path) -> Result<()> {
        let sftp = self.session.as_ref().unwrap().sftp()?;
        let capacity = 1024 * 1024;
        let mut fs = sftp.open(remote_path)?;

        let len = fs.stat()?.size.unwrap();
        let input_file = std::io::BufReader::with_capacity(capacity, fs);

        let file_name = remote_path.file_name().unwrap().to_str().unwrap();
        let pb = ProgressBar::new(len.clone());
        pb.set_style(ProgressStyle::default_bar()
            .template(&*format!("{0} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] {{bytes}}/{{total_bytes}} ({{bytes_per_sec}}, {{eta}})", file_name))
            .progress_chars("#>-"));
        let mut fs = OpenOptions::new().create(true).append(true).open(file_path)?;
        std::io::copy(&mut pb.wrap_read(input_file), &mut fs)?;
        pb.finish();
        Ok(())
    }

    pub fn file_list(&mut self, path: &Path) -> Result<Vec<(PathBuf, FileStat)>> {
        match self.session.as_ref().unwrap().sftp() {
            Ok(sftp) => {
                Ok(sftp.readdir(path)?)
            }
            Err(err) => Err(anyhow!(err.to_string()))
        }
    }

    pub fn check_is_dir(&mut self, path: &Path) -> Result<bool> {
        match self.session.as_ref().unwrap().sftp() {
            Ok(sftp) => {
                let stat = sftp.stat(path)?;
                Ok(stat.is_dir())
            }
            Err(err) => Err(anyhow!(err.to_string()))
        }
    }

    pub fn check_remote_dir(&mut self, path: &Path, create: bool) -> Result<()> {
        match self.session.as_ref().unwrap().sftp() {
            Ok(sftp) => {
                match sftp.stat(path) {
                    Err(_e) => {
                        if create {
                            Ok(self.exec(format!("mkdir -p {}", path.to_str().unwrap()))?)
                        } else {
                            Err(anyhow!("远程目录不存在！"))
                        }
                    }
                    Ok(_stat) => Ok(())
                }
            }
            Err(err) => Err(anyhow!(err.to_string()))
        }
    }
}