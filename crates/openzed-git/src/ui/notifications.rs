#![allow(unused)]

use crate::ui::aura::aura::{CROSS, TEXT, TEXT_DIM, TEXT_MUTED, YELLOW};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_bold, aura_text};

/// Error type enum for classification
#[derive(Debug, Clone)]
pub enum ErrorType {
    Internal,
    Git,
    GitHubCli,
    GitHubAuth,
    GitHubApi,
    Config,
    FileSystem,
    Zed,
    Unknown,
}

impl ErrorType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ErrorType::Internal => "Internal Error",
            ErrorType::Git => "Git Error",
            ErrorType::GitHubCli => "GitHub CLI Error",
            ErrorType::GitHubAuth => "GitHub Authentication Error",
            ErrorType::GitHubApi => "GitHub API Error",
            ErrorType::Config => "Config Error",
            ErrorType::FileSystem => "File System Error",
            ErrorType::Zed => "Zed Integration Error",
            ErrorType::Unknown => "Unknown Error",
        }
    }

    pub fn from_error(error_msg: &str, command: &str) -> Self {
        let msg_lower = error_msg.to_lowercase();

        // Check for git errors
        if command == "git" || msg_lower.contains("git") {
            return ErrorType::Git;
        }

        // Check for GitHub CLI errors
        if msg_lower.contains("gh ") || msg_lower.contains("github") {
            if msg_lower.contains("auth")
                || msg_lower.contains("login")
                || msg_lower.contains("authenticated")
            {
                return ErrorType::GitHubAuth;
            }
            if msg_lower.contains("api")
                || msg_lower.contains("rate limit")
                || msg_lower.contains("network")
            {
                return ErrorType::GitHubApi;
            }
            return ErrorType::GitHubCli;
        }

        // Check for config errors
        if msg_lower.contains("config") || msg_lower.contains("toml") {
            return ErrorType::Config;
        }

        // Check for filesystem errors
        if msg_lower.contains("permission")
            || msg_lower.contains("no such file")
            || msg_lower.contains("directory")
        {
            return ErrorType::FileSystem;
        }

        // Check for zed errors
        if msg_lower.contains("zed") {
            return ErrorType::Zed;
        }

        ErrorType::Unknown
    }
}

/// Error report struct
#[derive(Debug, Clone)]
pub struct ErrorReport {
    pub command: String,
    pub error_type: ErrorType,
    pub title: String,
    pub message: String,
    pub raw_error: String,
    pub suggested_fix: Option<String>,
    pub log_path: Option<std::path::PathBuf>,
}

impl ErrorReport {
    pub fn new(
        command: &str,
        error_type: ErrorType,
        title: &str,
        message: &str,
        raw_error: &str,
    ) -> Self {
        let suggested_fix = Self::suggest_fix(&error_type, message);

        Self {
            command: command.to_string(),
            error_type,
            title: title.to_string(),
            message: message.to_string(),
            raw_error: raw_error.to_string(),
            suggested_fix,
            log_path: None,
        }
    }

    fn suggest_fix(error_type: &ErrorType, message: &str) -> Option<String> {
        let msg_lower = message.to_lowercase();

        match error_type {
            ErrorType::GitHubCli => {
                if msg_lower.contains("not found") || msg_lower.contains("command not found") {
                    Some("Install GitHub CLI from https://cli.github.com/".to_string())
                } else {
                    Some("Run: gh auth login".to_string())
                }
            }
            ErrorType::GitHubAuth => Some("Run: gh auth login".to_string()),
            ErrorType::GitHubApi => {
                Some("Check your internet connection and try again.".to_string())
            }
            ErrorType::Git => {
                if msg_lower.contains("upstream") || msg_lower.contains("remote") {
                    Some("Run: openzed-git push".to_string())
                } else if msg_lower.contains("merge") || msg_lower.contains("conflict") {
                    Some("Run: openzed-git conflicts".to_string())
                } else if msg_lower.contains("rebase") {
                    Some("Run: openzed-git rebase-helper".to_string())
                } else if msg_lower.contains("repository") || msg_lower.contains("not a git") {
                    Some("Run: git init or open a Git project folder.".to_string())
                } else {
                    Some("Check git status for more information.".to_string())
                }
            }
            ErrorType::Config => {
                Some("Run: openzed-git config and recreate the config file.".to_string())
            }
            ErrorType::Zed => {
                Some("Install/enable Zed CLI or open files manually in Zed.".to_string())
            }
            ErrorType::FileSystem => Some("Check file permissions and path.".to_string()),
            ErrorType::Internal | ErrorType::Unknown => None,
        }
    }
}

/// Show error notification with Aura Dark TUI
pub fn show_error_notification(report: &ErrorReport) {
    separator("OpenZed Git: Error");

    println!();
    print!("  ");
    aura_text(CROSS.trim(), TEXT_MUTED);
    print!(" ");
    aura_bold(&report.title, TEXT);
    println!();

    println!();
    labeled_value("Command", &report.command);
    labeled_value("Error type", report.error_type.display_name());

    println!();
    print!("  ");
    aura_text("Message:", TEXT_MUTED);
    println!();
    print!("  ");
    aura_text(&report.message, TEXT);
    println!();

    if let Some(ref fix) = report.suggested_fix {
        println!();
        print!("  ");
        aura_text("Suggested fix:", TEXT_MUTED);
        println!();
        print!("  ");
        aura_text(fix, TEXT);
        println!();
    }

    if let Some(ref path) = report.log_path {
        println!();
        print!("  ");
        aura_text("Error log saved:", TEXT_MUTED);
        println!();
        print!("  ");
        aura_text(path.to_string_lossy().as_ref(), TEXT_DIM);
        println!();
    }
}

/// Show warning notification
pub fn show_warning_notification(title: &str, message: &str) {
    separator("OpenZed Git: Warning");

    println!();
    print!("  ");
    aura_text("⚠", TEXT_MUTED);
    print!(" ");
    aura_bold(title, TEXT);
    println!();

    println!();
    print!("  ");
    aura_text(message, TEXT);
    println!();
}

/// Show success notification
pub fn show_success_notification(title: &str, message: &str) {
    separator("OpenZed Git: Success");

    println!();
    print!("  ");
    aura_text("✓", TEXT_MUTED);
    print!(" ");
    aura_bold(title, TEXT);
    println!();

    if !message.is_empty() {
        println!();
        print!("  ");
        aura_text(message, TEXT);
        println!();
    }
}

fn labeled_value(label: &str, value: &str) {
    print!("  ");
    aura_text(label, TEXT_MUTED);
    print!(" ");
    aura_text(value, TEXT);
    println!();
}
