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
        // Format: hash<tab>date<tab>ref<tab>message
        let output = Command::new("git")
            .args([
                "log",
                "--oneline",
                "-15",
                "--decorate",
                "--date=short",
                "--pretty=format:%h%x09%ad%x09%D%x09%s",
            ])
            .output()
            .context("Failed to get git log")?;

        if !output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }

        let output = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output.lines().collect();

        if lines.is_empty() {
            return Ok("No commits yet".to_string());
        }

        let mut result = String::new();

        for (i, line) in lines.iter().enumerate() {
            // Split by tab: hash | date | refs | message
            let parts: Vec<&str> = line.splitn(4, '\t').collect();

            let (graph_sym, hash, date, refs, msg) = if parts.len() >= 4 {
                (
                    if i == 0 {
                        "\x1b[38;2;99;209;169m●\x1b[0m"
                    } else {
                        "\x1b[38;2;99;209;169m│\x1b[0m"
                    },
                    parts[0], // hash
                    parts[1], // date
                    parts[2], // refs (can be empty)
                    parts[3], // message
                )
            } else if parts.len() == 3 {
                (
                    if i == 0 {
                        "\x1b[38;2;99;209;169m●\x1b[0m"
                    } else {
                        "\x1b[38;2;99;209;169m│\x1b[0m"
                    },
                    parts[0],
                    parts[1],
                    "",
                    parts[2],
                )
            } else if parts.len() == 2 {
                (
                    if i == 0 {
                        "\x1b[38;2;99;209;169m●\x1b[0m"
                    } else {
                        "\x1b[38;2;99;209;169m│\x1b[0m"
                    },
                    parts[0],
                    "",
                    "",
                    parts[1],
                )
            } else {
                result.push_str(line);
                result.push('\n');
                continue;
            };

            // Format: ● hash  date  refs  message
            // Colors:  cyan  cyan  gray  purple/yellow  text
            result.push_str("  ");
            result.push_str(graph_sym);
            result.push_str(" ");

            // Hash in cyan
            result.push_str("\x1b[38;2;99;209;169m");
            result.push_str(hash);
            result.push_str("\x1b[0m ");

            // Date in muted gray
            if !date.is_empty() {
                result.push_str("\x1b[38;2;98;114;164m");
                result.push_str(date);
                result.push_str("\x1b[0m  ");
            } else {
                result.push_str("        ");
            }

            // Refs in purple/yellow
            if !refs.is_empty() {
                // Color branch names and refs
                let colored_refs = Self::color_branch_names(refs);

                result.push_str(&colored_refs);
                result.push_str("  ");
            }

            // Message in white/text
            result.push_str("\x1b[38;2;248;248;242m");
            result.push_str(msg);
            result.push_str("\x1b[0m");

            result.push('\n');
        }

        Ok(result.trim_end().to_string())
    }

    /// Color branch names in refs purple
    fn color_branch_names(refs: &str) -> String {
        let mut result = refs.to_string();

        // First, replace arrows and separators with colored versions
        const ARROW: &str = "\x1b[38;2;255;113;231m→\x1b[0m";
        result = result.replace(" -> ", &format!(" {} ", ARROW));
        result = result.replace(", ", &format!(",\x1b[0m "));

        // Color HEAD in yellow
        result = result.replace("HEAD", &format!("\x1b[38;2;255;228;106mHEAD\x1b[0m"));

        // Split by commas and color each branch name
        let parts: Vec<&str> = result.split(',').collect();
        let mut colored = Vec::new();

        for part in parts {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            // Check if this part is already colored (has ANSI codes)
            if part.contains("\x1b[") {
                colored.push(part.to_string());
            } else {
                // Color branch name in purple
                colored.push(format!("\x1b[38;2;162;119;255m{}\x1b[0m", part));
            }
        }

        colored.join(", ")
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
