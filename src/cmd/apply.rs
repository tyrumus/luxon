use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

/// apply changes from source to target
#[derive(Debug, Parser)]
pub struct Apply {
    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    etc: bool,
}

impl LuxonCommand for Apply {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        println!("Applying source to target");
        if self.all || self.etc {
            println!("Also apply the /etc stuff");
        }
        Ok(())
    }
}
