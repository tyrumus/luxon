mod utils;
mod cmd;
use crate::utils::{LuxonCli, LuxonCommand};

fn main() -> miette::Result<()> {
    let app = LuxonCli::new();
    app.cmd.exec()
}
