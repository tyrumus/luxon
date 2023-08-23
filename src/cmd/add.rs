use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

use std::path::PathBuf;

/// Add path
#[derive(Debug, Parser)]
pub struct Add {
    path: PathBuf,
    #[arg(short, long)]
    recursive: bool,
}

impl LuxonCommand for Add {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        println!("Adding path: {:?}", self.path);
        if self.recursive {
            println!("Add everything inside too");
        }
        Ok(())
    }
}
