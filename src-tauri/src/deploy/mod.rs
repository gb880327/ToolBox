use anyhow::{Result};
use tauri::Window;

use crate::cmd::CmdUtil;
use crate::cmd::ssh::SshUtil;

pub struct DeployUtil {
    pub cmd: CmdUtil,
    pub ssh: Option<SshUtil>,
    pub win: Window,
}



impl DeployUtil {
    pub fn new(win: Window) -> Result<DeployUtil> {
        let cmd = CmdUtil::new(win.clone());
        Ok(DeployUtil { cmd, ssh: None, win })
    }

    pub fn deploy(&mut self) -> Result<()> {

        Ok(())
    }
}