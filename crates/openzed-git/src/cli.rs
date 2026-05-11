use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(
    name = "openzed-git",
    about = "Extra Git tools for Zed",
    long_about = None,
    version = VERSION
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug, Clone)]
pub enum Commands {
    /// Publish local project to GitHub
    Publish,
    /// Show Git graph / commit history
    Graph,
    /// Show richer Git status
    StatusPlus,
    /// Commit staged files and push
    CommitPush,
    /// Push and optionally set upstream
    Push,
    /// Pull from remote
    Pull,
    /// List branches
    Branches,
    /// List remotes
    Remotes,
    /// Setup remote origin
    SetupRemote,
    /// Rename current branch
    RenameBranch,
    /// Set upstream branch
    SetUpstream,
    /// Check Git setup and configuration
    Doctor,
    /// Open GitHub repository in browser
    OpenGithub,
    /// Install OpenZed Git tasks in Zed
    InstallZedTasks,
}
