use crate::cli::Commands;
use anyhow::Result;

pub mod branches;
pub mod commit;
pub mod commit_push;
pub mod config;
pub mod conflicts;
pub mod doctor;
pub mod graph;
pub mod install_keybindings;
pub mod log;
pub mod install_zed_tasks;
pub mod menu;
pub mod open_github;
pub mod pr_checkout;
pub mod pr_create;
pub mod pr_list;
pub mod pr_open;
pub mod publish;
pub mod pull;
pub mod push;
pub mod rebase_helper;
pub mod remotes;
pub mod rename_branch;
pub mod restore_file;
pub mod set_upstream;
pub mod setup_remote;
pub mod stash;
pub mod status_plus;
pub mod switch_branch;
pub mod theme;
pub mod undo_last_commit;
pub mod unstage;

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
        Commands::Stash => stash::run(),
        Commands::Switch => switch_branch::run(),
        Commands::UndoLastCommit => undo_last_commit::run(),
        Commands::Unstage => unstage::run(),
        Commands::RestoreFile => restore_file::run(),
        Commands::Commit => commit::run(),
        Commands::PrCreate => pr_create::run(),
        Commands::PrList => pr_list::run(),
        Commands::PrCheckout => pr_checkout::run(),
        Commands::PrOpen => pr_open::run(),
        Commands::Conflicts => conflicts::run(),
        Commands::RebaseHelper => rebase_helper::run(),
        Commands::Menu => menu::run(),
        Commands::Config => config::run(),
        Commands::Theme => theme::run(),
        Commands::InstallKeybindings => install_keybindings::run(),
        Commands::Log => log::run(),
    }
}
