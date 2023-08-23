use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

/// List files unmanaged by luxon
#[derive(Debug, Parser)]
pub struct Unmanaged {}

impl LuxonCommand for Unmanaged {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        println!("Files unmanaged by luxon:");
        Ok(())
    }
}
