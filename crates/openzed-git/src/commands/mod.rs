use crate::cli::Commands;
use anyhow::Result;

pub mod branches;
pub mod commit_push;
pub mod doctor;
pub mod graph;
pub mod install_zed_tasks;
pub mod open_github;
pub mod publish;
pub mod pull;
pub mod push;
pub mod remotes;
pub mod rename_branch;
pub mod set_upstream;
pub mod setup_remote;
pub mod status_plus;

pub fn execute(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Publish => publish::run(),
        Commands::Graph => graph::run(),
        Commands::StatusPlus => status_plus::run(),
        Commands::CommitPush => commit_push::run(),
        Commands::Push => push::run(),
        Commands::Pull => pull::run(),
        Commands::Branches => branches::run(),
        Commands::Remotes => remotes::run(),
        Commands::SetupRemote => setup_remote::run(),
        Commands::RenameBranch => rename_branch::run(),
        Commands::SetUpstream => set_upstream::run(),
        Commands::OpenGithub => open_github::run(),
        Commands::Doctor => doctor::run(),
        Commands::InstallZedTasks => install_zed_tasks::run(),
    }
}
