use crate::utils::LuxonCommand;
use crate::utils::xdg;
use clap::Parser;
use miette::Result;
use std::env;
use std::process::Command;

/// spawn shell inside luxon folder
#[derive(Debug, Parser)]
pub struct Cd {}

impl LuxonCommand for Cd {
    fn exec(self) -> Result<()> {
        let shell_bin = xdg::get_shell();

        let local_path = xdg::get_repo_dir();
        if env::set_current_dir(&local_path).is_ok() {
            Command::new(shell_bin).spawn().unwrap().wait().unwrap();
        }else{
            println!("failed to cd");
        }
        Ok(())
    }
}
