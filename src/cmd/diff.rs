use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

/// Run diff of everything
#[derive(Debug, Parser)]
pub struct Diff {
    #[arg(short, long)]
    all: bool,
    #[arg(short, long)]
    etc: bool,
}

impl LuxonCommand for Diff {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        if self.all || self.etc {
            println!("Running diff of all files");
        } else {
            println!("Running diff of home dir files");
        }
        Ok(())
    }
}
