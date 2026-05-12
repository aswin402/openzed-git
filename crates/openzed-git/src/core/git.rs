#![allow(unused)]

use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub is_repo: bool,
    pub current_branch: Option<String>,
    pub remote_origin: Option<String>,
    pub has_commits: bool,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
}

impl GitInfo {
    pub fn get() -> Result<Self> {
        let is_repo = Self::is_repo()?;
        let current_branch = Self::current_branch().ok();
        let remote_origin = Self::remote_origin().ok();
        let has_commits = Self::has_commits().unwrap_or(false);
        let user_name = Self::user_name().ok();
        let user_email = Self::user_email().ok();

        Ok(Self {
            is_repo,
            current_branch,
            remote_origin,
            has_commits,
            user_name,
            user_email,
        })
    }

    pub fn is_repo() -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .context("Failed to run git")?;

        Ok(output.status.success())
    }

    pub fn current_branch() -> Result<String> {
        let output = Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .context("Failed to get current branch")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            anyhow::bail!("Not on a branch")
        }
    }

    pub fn remote_origin() -> Result<String> {
        let output = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()
            .context("Failed to get remote origin")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            anyhow::bail!("No remote origin")
        }
    }

    pub fn has_commits() -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .context("Failed to check for commits")?;

        Ok(output.status.success())
    }

    pub fn user_name() -> Result<String> {
        let output = Command::new("git")
            .args(["config", "--get", "user.name"])
            .output()
            .context("Failed to get user.name")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            anyhow::bail!("user.name not set")
        }
    }

    pub fn user_email() -> Result<String> {
        let output = Command::new("git")
            .args(["config", "--get", "user.email"])
            .output()
            .context("Failed to get user.email")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            anyhow::bail!("user.email not set")
        }
    }

    pub fn upstream() -> Result<Option<String>> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "@{upstream}"])
            .output()
            .context("Failed to get upstream")?;

        if output.status.success() {
            let upstream = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if upstream.is_empty() || upstream == "@{upstream}" {
                Ok(None)
            } else {
                Ok(Some(upstream))
            }
        } else {
            Ok(None)
        }
    }

    pub fn init() -> Result<()> {
        Command::new("git")
            .arg("init")
            .output()
            .context("Failed to run git init")?;

        Ok(())
    }

    pub fn add_all() -> Result<()> {
        Command::new("git")
            .arg("add")
            .arg(".")
            .output()
            .context("Failed to run git add")?;

        Ok(())
    }

    pub fn commit(message: &str) -> Result<()> {
        Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .context("Failed to commit")?;

        Ok(())
    }

    pub fn branch_rename(old: &str, new: &str) -> Result<()> {
        Command::new("git")
            .args(["branch", "-m", old, new])
            .output()
            .context("Failed to rename branch")?;

        Ok(())
    }

    pub fn push_u(origin: &str, branch: &str) -> Result<()> {
        Command::new("git")
            .args(["push", "-u", origin, branch])
            .output()
            .context("Failed to push and set upstream")?;

        Ok(())
    }

    pub fn push() -> Result<()> {
        Command::new("git")
            .arg("push")
            .output()
            .context("Failed to push")?;

        Ok(())
    }

    pub fn pull() -> Result<()> {
        Command::new("git")
            .arg("pull")
            .output()
            .context("Failed to pull")?;

        Ok(())
    }

    pub fn remote_add(name: &str, url: &str) -> Result<()> {
        Command::new("git")
            .args(["remote", "add", name, url])
            .output()
            .context("Failed to add remote")?;

        Ok(())
    }

    pub fn branch(branch: &str) -> Result<()> {
        Command::new("git")
            .args(["branch", "-M", branch])
            .output()
            .context("Failed to create branch")?;

        Ok(())
    }

    pub fn status() -> Result<String> {
        let output = Command::new("git")
            .args(["status", "--porcelain=v1", "-b"])
            .output()
            .context("Failed to get git status")?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn log_graph() -> Result<String> {
        let output = Command::new("git")
            .args([
                "log",
                "--graph",
                "--decorate",
                "--oneline",
                "--all",
                "--date=relative",
                r#"--pretty=format:"%h%x09%an%x09%ar%x09%d%x09%s""#,
            ])
            .output()
            .context("Failed to get git log")?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn log_simple() -> Result<String> {
        let output = Command::new("git")
            .args(["log", "--oneline", "-15", "--decorate", "--date=relative"])
            .output()
            .context("Failed to get git log")?;

        if !output.status.success() {
            return Ok("No commits yet".to_string());
        }

        let output = String::from_utf8_lossy(&output.stdout);

        let lines: Vec<&str> = output.lines().collect();
        if lines.is_empty() {
            return Ok("No commits yet".to_string());
        }

        let mut result = String::new();

        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                result.push_str(&format!("  \x1b[38;2;0;255;135m•\x1b[0m {}\n", line));
            } else {
                result.push_str(&format!("  \x1b[38;2;80;80;80m│\x1b[0m {}\n", line));
            }
        }

        Ok(result.trim_end().to_string())
    }

    pub fn branches() -> Result<String> {
        let output = Command::new("git")
            .args(["branch", "--all", "--verbose"])
            .output()
            .context("Failed to list branches")?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn remotes() -> Result<String> {
        let output = Command::new("git")
            .args(["remote", "-v"])
            .output()
            .context("Failed to list remotes")?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
