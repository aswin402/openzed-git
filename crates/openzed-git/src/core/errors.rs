use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenZedGitError {
    #[error("Git is not installed")]
    GitNotInstalled,

    #[error("Not a Git repository")]
    NotAGitRepository,

    #[error("GitHub CLI is not installed")]
    GhNotInstalled,

    #[error("GitHub CLI is not authenticated")]
    GhNotAuthenticated,

    #[error("No remote origin found")]
    NoRemoteOrigin,

    #[error("Git user.name not configured")]
    UserNameNotConfigured,

    #[error("Git user.email not configured")]
    UserEmailNotConfigured,

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Failed to create repository: {0}")]
    RepoCreationFailed(String),
}
