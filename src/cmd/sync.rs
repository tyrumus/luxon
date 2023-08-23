use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

/// Sync repo with remote and apply changes
#[derive(Debug, Parser)]
pub struct Sync {}

impl LuxonCommand for Sync {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        println!("Syncing repo with remote");
        Ok(())
    }
}
