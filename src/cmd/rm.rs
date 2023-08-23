use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

use std::path::PathBuf;

/// Remove path
#[derive(Debug, Parser)]
pub struct Rm {
    path: PathBuf,
    #[arg(short, long)]
    recursive: bool,
}

impl LuxonCommand for Rm {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        println!("Removing path: {:?}", self.path);
        if self.recursive {
            println!("Remove everything inside too");
        }
        Ok(())
    }
}
