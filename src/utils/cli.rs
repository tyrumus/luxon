use crate::cmd::{add,apply,cd,completion,diff,exectemp,init,install,managed,pull,rm,sshremote,sync,unmanaged};
use clap::{Parser, Subcommand};

pub trait LuxonCommand {
    fn exec(self) -> miette::Result<()>;
}

/// Manage your dotfiles, system configs, and packages across multiple machines
#[derive(Debug, Subcommand)]
pub enum LuxonSubCmd {
    Add(add::Add),
    Apply(apply::Apply),
    Cd(cd::Cd),
    Completion(completion::Completion),
    Diff(diff::Diff),
    ExecTemp(exectemp::ExecTemp),
    Init(init::Init),
    Install(install::Install),
    Managed(managed::Managed),
    Pull(pull::Pull),
    Rm(rm::Rm),
    SshRemote(sshremote::SshRemote),
    Sync(sync::Sync),
    Unmanaged(unmanaged::Unmanaged),
}

impl LuxonCommand for LuxonSubCmd {
    fn exec(self) -> miette::Result<()> {
        match self {
            Self::Add(x) => x.exec(),
            Self::Apply(x) => x.exec(),
            Self::Cd(x) => x.exec(),
            Self::Completion(x) => x.exec(),
            Self::Diff(x) => x.exec(),
            Self::ExecTemp(x) => x.exec(),
            Self::Init(x) => x.exec(),
            Self::Install(x) => x.exec(),
            Self::Managed(x) => x.exec(),
            Self::Pull(x) => x.exec(),
            Self::Rm(x) => x.exec(),
            Self::SshRemote(x) => x.exec(),
            Self::Sync(x) => x.exec(),
            Self::Unmanaged(x) => x.exec(),
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "luxon")]
pub struct LuxonCli {
    #[command(subcommand)]
    pub cmd: LuxonSubCmd,
}

impl LuxonCli {
    pub fn new() -> Self {
        Self::parse()
    }
}
