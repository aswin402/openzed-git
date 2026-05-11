use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GithubInfo {
    pub gh_installed: bool,
    pub gh_authenticated: bool,
    pub gh_user: Option<String>,
}

impl GithubInfo {
    pub fn get() -> Result<Self> {
        let gh_installed = Self::is_installed()?;
        let gh_authenticated = Self::is_authenticated()?;
        let gh_user = Self::current_user().ok();

        Ok(Self {
            gh_installed,
            gh_authenticated,
            gh_user,
        })
    }

    pub fn is_installed() -> Result<bool> {
        let output = Command::new("gh")
            .args(["--version"])
            .output()
            .context("Failed to check gh")?;

        Ok(output.status.success())
    }

    pub fn is_authenticated() -> Result<bool> {
        let output = Command::new("gh")
            .args(["auth", "status"])
            .output()
            .context("Failed to check gh auth status")?;

        Ok(output.status.success())
    }

    pub fn current_user() -> Result<String> {
        let output = Command::new("gh")
            .args(["api", "user"])
            .output()
            .context("Failed to get GitHub user")?;

        if output.status.success() {
            let json: serde_json::Value = serde_json::from_slice(&output.stdout)
                .context("Failed to parse gh user response")?;
            Ok(json["login"].as_str().unwrap_or("unknown").to_string())
        } else {
            anyhow::bail!("Not authenticated with GitHub")
        }
    }

    pub fn create_repo(
        name: &str,
        description: Option<&str>,
        public: bool,
        cwd: &Path,
    ) -> Result<String> {
        let cwd_str = cwd
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
        let mut args = vec![
            "repo",
            "create",
            name,
            "--source",
            cwd_str,
            "--remote=origin",
            "--push",
        ];
        if public {
            args.push("--public");
        } else {
            args.push("--private");
        }
        if let Some(desc) = description {
            if !desc.is_empty() {
                args.extend(["--description", desc]);
            }
        }
        let output = Command::new("gh")
            .args(&args)
            .output()
            .context("Failed to create GitHub repository")?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to create repo: {}", stderr)
        }
    }

    pub fn push() -> Result<()> {
        Command::new("git")
            .args(["push", "-u", "origin", "HEAD"])
            .output()
            .context("Failed to push to remote")?;
        Ok(())
    }

    pub fn remote_to_github_url(remote: &str) -> Option<String> {
        let remote = remote.trim();

        if remote.starts_with("git@github.com:") {
            let repo_part = remote.trim_start_matches("git@github.com:");
            let repo_part = repo_part.strip_suffix(".git").unwrap_or(repo_part);
            Some(format!("https://github.com/{}", repo_part))
        } else if remote.starts_with("https://github.com/") {
            let repo_part = remote.strip_suffix(".git").unwrap_or(remote);
            Some(repo_part.to_string())
        } else {
            None
        }
    }
}
