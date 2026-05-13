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
    /// Manage Git stashes
    Stash,
    /// Switch branches interactively
    Switch,
    /// Undo last commit safely
    UndoLastCommit,
    /// Unstage files
    Unstage,
    /// Restore files to last commit
    RestoreFile,
    /// Guided conventional commit assistant
    Commit,
    /// Create a pull request on GitHub
    PrCreate,
    /// List open pull requests
    PrList,
    /// Checkout a pull request locally
    PrCheckout,
    /// Open a pull request in browser
    PrOpen,
    /// Help resolve merge conflicts
    Conflicts,
    /// Help with Git rebase operations
    RebaseHelper,
    /// Open interactive menu
    Menu,
    /// Manage configuration
    Config,
    /// Manage themes
    Theme,
    /// Install Zed keybindings
    InstallKeybindings,
    /// View error logs
    Log,
}
