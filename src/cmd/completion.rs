use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

/// Generate shell completion code
#[derive(Debug, Parser)]
pub struct Completion {
    shell: String,
}

impl LuxonCommand for Completion {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        println!("Generating completion code for {}", self.shell);
        Ok(())
    }
}
