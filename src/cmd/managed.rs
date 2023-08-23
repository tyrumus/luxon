use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

/// List files managed by luxon
#[derive(Debug, Parser)]
pub struct Managed {}

impl LuxonCommand for Managed {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        println!("Files managed by luxon:");
        Ok(())
    }
}
