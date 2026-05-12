#![allow(unused)]

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

    pub fn ensure_gh_ready() -> Result<()> {
        if !Self::is_installed()? {
            anyhow::bail!("GitHub CLI is not installed");
        }
        if !Self::is_authenticated()? {
            anyhow::bail!("GitHub CLI is not authenticated");
        }
        Ok(())
    }

    pub fn pr_list() -> Result<Vec<PullRequest>> {
        let output = Command::new("gh")
            .args([
                "pr",
                "list",
                "--limit",
                "20",
                "--json",
                "number,title,headRefName,baseRefName,state,isDraft,author,updatedAt,url",
            ])
            .output()
            .context("Failed to list PRs")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to list PRs: {}", stderr.trim())
        }

        let json: Vec<serde_json::Value> =
            serde_json::from_slice(&output.stdout).context("Failed to parse PR list response")?;

        let prs: Vec<PullRequest> = json
            .into_iter()
            .map(|pr| PullRequest {
                number: pr["number"].as_u64().unwrap_or(0) as u32,
                title: pr["title"].as_str().unwrap_or("").to_string(),
                head: pr["headRefName"].as_str().unwrap_or("").to_string(),
                base: pr["baseRefName"].as_str().unwrap_or("").to_string(),
                state: pr["state"].as_str().unwrap_or("").to_string(),
                is_draft: pr["isDraft"].as_bool().unwrap_or(false),
                author: pr["author"]["login"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string(),
                updated_at: pr["updatedAt"].as_str().unwrap_or("").to_string(),
                url: pr["url"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        Ok(prs)
    }

    pub fn pr_create(
        base: &str,
        head: &str,
        title: &str,
        body: Option<&str>,
        draft: bool,
    ) -> Result<String> {
        let mut args = vec![
            "pr", "create", "--base", base, "--head", head, "--title", title,
        ];

        if let Some(b) = body {
            if !b.is_empty() {
                args.extend(["--body", b]);
            }
        }

        if draft {
            args.push("--draft");
        }

        let output = Command::new("gh")
            .args(&args)
            .output()
            .context("Failed to create PR")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to create PR: {}", stderr.trim())
        }

        // PR URL is usually in stdout
        let stdout = String::from_utf8_lossy(&output.stdout);
        let url = stdout.trim();
        Ok(url.to_string())
    }

    pub fn pr_checkout(number: u32) -> Result<()> {
        let output = Command::new("gh")
            .args(["pr", "checkout", &number.to_string()])
            .output()
            .context("Failed to checkout PR")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to checkout PR: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn pr_view_web(number: u32) -> Result<()> {
        let output = Command::new("gh")
            .args(["pr", "view", &number.to_string(), "--web"])
            .output()
            .context("Failed to open PR in browser")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to open PR: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn pr_view_web_current() -> Result<()> {
        let output = Command::new("gh")
            .args(["pr", "view", "--web"])
            .output()
            .context("Failed to open PR in browser")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to open PR: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn current_pr_url() -> Result<Option<String>> {
        let output = Command::new("gh")
            .args(["pr", "view", "--json", "url"])
            .output()
            .context("Failed to get current PR")?;

        if !output.status.success() {
            return Ok(None);
        }

        let json: serde_json::Value =
            serde_json::from_slice(&output.stdout).context("Failed to parse PR response")?;

        let url = json["url"].as_str().unwrap_or("").to_string();
        if url.is_empty() {
            Ok(None)
        } else {
            Ok(Some(url))
        }
    }

    pub fn push_current_branch() -> Result<()> {
        let output = Command::new("git")
            .args(["push", "-u", "origin", "HEAD"])
            .output()
            .context("Failed to push current branch")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to push: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn has_upstream() -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "@{upstream}"])
            .output()
            .context("Failed to check upstream")?;

        Ok(output.status.success())
    }
}

#[derive(Debug, Clone)]
pub struct PullRequest {
    pub number: u32,
    pub title: String,
    pub head: String,
    pub base: String,
    pub state: String,
    pub is_draft: bool,
    pub author: String,
    pub updated_at: String,
    pub url: String,
}
