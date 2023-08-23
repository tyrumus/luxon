use crate::utils::LuxonCommand;
use clap::Parser;
use miette::Result;

/// Install packages
#[derive(Debug, Parser)]
pub struct Install {}

impl LuxonCommand for Install {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        println!("Install all the things!!!");

        // build package list

        // do a diff of installed packages vs desired packages
        // OPTIONAL: warn user of packages that were installed by luxon, but no longer in repo

        // install packages not currently installed

        // run post install scripts
        Ok(())
    }
}
