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
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .context("Failed to commit")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let error_msg = if stderr.is_empty() { stdout.as_ref() } else { stderr.as_ref() };
            anyhow::bail!("Commit failed: {}", error_msg.trim());
        }

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
        let output = Command::new("git")
            .args(["push", "-u", origin, branch])
            .output()
            .context("Failed to push and set upstream")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Push failed: {}", stderr);
        }

        Ok(())
    }

    pub fn push() -> Result<()> {
        let output = Command::new("git")
            .arg("push")
            .output()
            .context("Failed to push")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Push failed: {}", stderr);
        }

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

    pub fn stash_push(message: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["stash", "push", "-m", message])
            .output()
            .context("Failed to stash changes")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to stash: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn stash_list() -> Result<Vec<StashEntry>> {
        let output = Command::new("git")
            .args(["stash", "list"])
            .output()
            .context("Failed to list stashes")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut stashes = Vec::new();

        for line in stdout.lines() {
            if let Some(entry) = StashEntry::parse(line) {
                stashes.push(entry);
            }
        }

        Ok(stashes)
    }

    pub fn stash_pop(index: usize) -> Result<()> {
        let stash_ref = format!("stash@{{{}}}", index);
        let output = Command::new("git")
            .args(["stash", "pop", &stash_ref])
            .output()
            .context("Failed to pop stash")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to pop stash: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn stash_apply(index: usize) -> Result<()> {
        let stash_ref = format!("stash@{{{}}}", index);
        let output = Command::new("git")
            .args(["stash", "apply", &stash_ref])
            .output()
            .context("Failed to apply stash")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to apply stash: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn stash_drop(index: usize) -> Result<()> {
        let stash_ref = format!("stash@{{{}}}", index);
        let output = Command::new("git")
            .args(["stash", "drop", &stash_ref])
            .output()
            .context("Failed to drop stash")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to drop stash: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn has_changes() -> Result<bool> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .context("Failed to check for changes")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(!stdout.trim().is_empty())
    }

    pub fn local_branches() -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["branch"])
            .output()
            .context("Failed to list local branches")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let branches: Vec<String> = stdout
            .lines()
            .map(|l| l.trim().trim_start_matches("* ").to_string())
            .filter(|b| !b.is_empty())
            .collect();

        Ok(branches)
    }

    pub fn remote_branches() -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["branch", "-r"])
            .output()
            .context("Failed to list remote branches")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let branches: Vec<String> = stdout
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|b| !b.is_empty() && !b.contains("->") && !b.contains("origin/HEAD"))
            .collect();

        Ok(branches)
    }

    pub fn branch_exists(name: &str) -> Result<bool> {
        let output = Command::new("git")
            .args(["rev-parse", &format!("refs/heads/{}", name)])
            .output()?;

        Ok(output.status.success())
    }

    pub fn switch_branch(name: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["switch", name])
            .output()
            .context("Failed to switch branch")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to switch branch: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn create_and_switch_branch(name: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["switch", "-c", name])
            .output()
            .context("Failed to create and switch branch")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to create branch: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn checkout_remote_branch(remote_branch: &str) -> Result<()> {
        // Extract local branch name from remote branch (e.g., "origin/feature" -> "feature")
        let local_name = remote_branch
            .split('/')
            .skip(1)
            .collect::<Vec<_>>()
            .join("/");

        let output = Command::new("git")
            .args(["switch", "--track", remote_branch])
            .output()
            .context("Failed to checkout remote branch")?;

        if !output.status.success() {
            // If it failed, it might be because local branch already exists
            // Try just switching to it
            if !local_name.is_empty() {
                return Self::switch_branch(&local_name);
            }
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to checkout remote branch: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn last_commit_summary() -> Result<CommitSummary> {
        let output = Command::new("git")
            .args([
                "log",
                "-1",
                "--pretty=format:%h%x09%s%x09%ad",
                "--date=short",
            ])
            .output()
            .context("Failed to get last commit")?;

        if !output.status.success() {
            anyhow::bail!("Failed to get last commit");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = stdout.splitn(3, '\t').collect();

        if parts.len() >= 3 {
            Ok(CommitSummary {
                hash: parts[0].to_string(),
                message: parts[1].to_string(),
                date: parts[2].to_string(),
            })
        } else if parts.len() == 2 {
            Ok(CommitSummary {
                hash: parts[0].to_string(),
                message: parts[1].to_string(),
                date: String::new(),
            })
        } else {
            Ok(CommitSummary {
                hash: parts[0].to_string(),
                message: parts.get(0).unwrap_or(&" ").to_string(),
                date: String::new(),
            })
        }
    }

    pub fn undo_last_commit_soft() -> Result<()> {
        let output = Command::new("git")
            .args(["reset", "--soft", "HEAD~1"])
            .output()
            .context("Failed to undo last commit (soft)")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to undo commit: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn undo_last_commit_mixed() -> Result<()> {
        let output = Command::new("git")
            .args(["reset", "--mixed", "HEAD~1"])
            .output()
            .context("Failed to undo last commit (mixed)")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to undo commit: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn staged_files() -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["diff", "--name-only", "--staged"])
            .output()
            .context("Failed to get staged files")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<String> = stdout
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        Ok(files)
    }

    pub fn unstage_files(files: &[String]) -> Result<()> {
        for file in files {
            let output = Command::new("git")
                .args(["restore", "--staged", file])
                .output()
                .context("Failed to unstage file")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("Failed to unstage {}: {}", file, stderr.trim())
            }
        }

        Ok(())
    }

    pub fn modified_unstaged_files() -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["diff", "--name-only"])
            .output()
            .context("Failed to get modified files")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<String> = stdout
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        Ok(files)
    }

    pub fn restore_files(files: &[String]) -> Result<()> {
        for file in files {
            let output = Command::new("git")
                .args(["restore", file])
                .output()
                .context("Failed to restore file")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("Failed to restore {}: {}", file, stderr.trim())
            }
        }

        Ok(())
    }

    pub fn changed_files() -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .context("Failed to get changed files")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<String> = stdout
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .map(|l| l[3..].to_string()) // Remove status columns like " M "
            .collect();

        Ok(files)
    }

    pub fn stage_files(files: &[String]) -> Result<()> {
        for file in files {
            let output = Command::new("git")
                .args(["add", file])
                .output()
                .context("Failed to stage file")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("Failed to stage {}: {}", file, stderr.trim())
            }
        }

        Ok(())
    }

    pub fn stage_all() -> Result<()> {
        let output = Command::new("git")
            .arg("add")
            .arg(".")
            .output()
            .context("Failed to stage all files")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to stage all files: {}", stderr.trim())
        }

        Ok(())
    }

    pub fn create_commit(message: &str, body: Option<&str>) -> Result<()> {
        if body.is_some() {
            let output = Command::new("git")
                .args(["commit", "-m", message, "-m", body.unwrap()])
                .output()
                .context("Failed to create commit")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("Failed to create commit: {}", stderr.trim())
            }
        } else {
            let output = Command::new("git")
                .args(["commit", "-m", message])
                .output()
                .context("Failed to create commit")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("Failed to create commit: {}", stderr.trim())
            }
        }

        Ok(())
    }

    pub fn last_commit_hash() -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .context("Failed to get last commit hash")?;

        if !output.status.success() {
            anyhow::bail!("Failed to get last commit hash")
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn conflicted_files() -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["diff", "--name-only", "--diff-filter=U"])
            .output()
            .context("Failed to get conflicted files")?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(files)
    }

    pub fn has_merge_in_progress() -> bool {
        let output = Command::new("git")
            .args(["rev-parse", "MERGE_HEAD"])
            .output();

        output.map(|o| o.status.success()).unwrap_or(false)
    }

    pub fn has_rebase_in_progress() -> bool {
        // Check for rebase-merge or rebase-apply directories
        std::path::Path::new(".git/rebase-merge").exists()
            || std::path::Path::new(".git/rebase-apply").exists()
    }

    pub fn mark_resolved(files: &[String]) -> Result<()> {
        for file in files {
            Command::new("git")
                .args(["add", file])
                .output()
                .context(format!("Failed to stage file: {}", file))?;
        }
        Ok(())
    }

    pub fn continue_merge() -> Result<()> {
        Command::new("git")
            .args(["merge", "--continue"])
            .output()
            .context("Failed to continue merge")?;

        Ok(())
    }

    pub fn abort_merge() -> Result<()> {
        Command::new("git")
            .args(["merge", "--abort"])
            .output()
            .context("Failed to abort merge")?;

        Ok(())
    }

    pub fn continue_rebase() -> Result<()> {
        Command::new("git")
            .args(["rebase", "--continue"])
            .output()
            .context("Failed to continue rebase")?;

        Ok(())
    }

    pub fn skip_rebase() -> Result<()> {
        Command::new("git")
            .args(["rebase", "--skip"])
            .output()
            .context("Failed to skip commit during rebase")?;
        Ok(())
    }

    pub fn abort_rebase() -> Result<()> {
        Command::new("git")
            .args(["rebase", "--abort"])
            .output()
            .context("Failed to abort rebase")?;
        Ok(())
    }

    pub fn open_in_zed(files: &[String]) -> Result<()> {
        for file in files {
            let output = Command::new("zed").arg(file).output();

            if output.is_err() {
                anyhow::bail!("Failed to open zed for file: {}", file);
            }
        }
        Ok(())
    }

    pub fn zed_available() -> bool {
        Command::new("zed")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GitState {
    Clean,
    MergeInProgress,
    RebaseInProgress,
    Conflicts,
}

#[derive(Debug, Clone)]
pub struct CommitSummary {
    pub hash: String,
    pub message: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct StashEntry {
    pub index: usize,
    pub branch: String,
    pub message: String,
}

impl StashEntry {
    fn parse(line: &str) -> Option<Self> {
        // Format: stash@{0}: WIP on <branch>: <hash> <message>
        // Or: stash@{0}: On <branch>: <message>

        let line = line.trim();
        if !line.starts_with("stash@") {
            return None;
        }

        // Extract index
        let index_start = line.find("{")? + 1;
        let index_end = line.find("}")?;
        let index_str = &line[index_start..index_end];
        let index: usize = index_str.parse().ok()?;

        // Extract branch and message
        // Find the colon after the index
        let after_index = &line[index_end + 1..];
        let colon_pos = after_index.find(':')?;

        let rest = &after_index[colon_pos + 1..].trim();

        // Try to extract branch from patterns like "WIP on main:" or "On main:"
        let (branch, message) = if rest.starts_with("WIP on ") {
            let after_wip = &rest[7..];
            if let Some(colon) = after_wip.find(':') {
                let branch = after_wip[..colon].to_string();
                let message = after_wip[colon + 1..].trim().to_string();
                (branch, message)
            } else {
                (String::from("unknown"), rest.to_string())
            }
        } else if rest.starts_with("On ") {
            let after_on = &rest[3..];
            if let Some(colon) = after_on.find(':') {
                let branch = after_on[..colon].to_string();
                let message = after_on[colon + 1..].trim().to_string();
                (branch, message)
            } else {
                (after_on.to_string(), String::new())
            }
        } else {
            (String::from("unknown"), rest.to_string())
        };

        Some(Self {
            index,
            branch,
            message,
        })
    }
}
