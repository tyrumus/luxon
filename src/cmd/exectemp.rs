use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

// use handlebars::Handlebars;

/// some shit about templates
#[derive(Debug, Parser)]
pub struct ExecTemp {
    temp: String,
}

impl LuxonCommand for ExecTemp {
    fn exec(self) -> Result<()> {
        // let reg = Handlebars::new();
        // TODO: provide template variables
        println!("Executing template: {:?}", self.temp);
        Ok(())
    }
}
