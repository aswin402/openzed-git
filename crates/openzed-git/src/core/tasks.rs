use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub label: String,
    pub command: String,
    #[serde(rename = "use_new_terminal")]
    pub use_new_terminal: bool,
    #[serde(rename = "allow_concurrent_runs")]
    pub allow_concurrent_runs: bool,
    pub reveal: String,
}

pub fn get_openzed_git_tasks() -> Vec<Task> {
    vec![
        Task {
            label: "OpenZed Git: Publish to GitHub".to_string(),
            command: "openzed-git publish".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Git Graph".to_string(),
            command: "openzed-git graph".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Status+".to_string(),
            command: "openzed-git status-plus".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Commit + Push".to_string(),
            command: "openzed-git commit-push".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Push / Set Upstream".to_string(),
            command: "openzed-git push".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Pull".to_string(),
            command: "openzed-git pull".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Branches".to_string(),
            command: "openzed-git branches".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Remotes".to_string(),
            command: "openzed-git remotes".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Setup Remote".to_string(),
            command: "openzed-git setup-remote".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Rename Branch".to_string(),
            command: "openzed-git rename-branch".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Set Upstream".to_string(),
            command: "openzed-git set-upstream".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Open GitHub Repo".to_string(),
            command: "openzed-git open-github".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
        Task {
            label: "OpenZed Git: Doctor".to_string(),
            command: "openzed-git doctor".to_string(),
            use_new_terminal: true,
            allow_concurrent_runs: false,
            reveal: "always".to_string(),
        },
    ]
}
