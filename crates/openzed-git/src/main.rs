mod cli;
mod commands;
mod core;
mod ui;

use crate::core::error_logger::{append_error_log, collect_error_context};
use crate::ui::banner::show_banner;
use crate::ui::notifications::{show_error_notification, ErrorReport, ErrorType};
use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() {
    if let Err(err) = run() {
        // Classify the error
        let error_msg = err.to_string();
        let command = determine_command();
        let error_type = ErrorType::from_error(&error_msg, &command);

        // Get title based on error type
        let title = match error_type {
            ErrorType::Git => "Git command failed",
            ErrorType::GitHubCli => "GitHub CLI error",
            ErrorType::GitHubAuth => "GitHub authentication failed",
            ErrorType::GitHubApi => "GitHub API error",
            ErrorType::Config => "Configuration error",
            ErrorType::FileSystem => "File system error",
            ErrorType::Zed => "Zed integration error",
            ErrorType::Internal => "Internal error",
            ErrorType::Unknown => "Unknown error occurred",
        };

        // Extract clean message
        let message = extract_clean_message(&error_msg);

        // Create error report
        let report = ErrorReport::new(&command, error_type, title, &message, &error_msg);

        // Collect context and log error
        let context = collect_error_context();
        let log_path = append_error_log(
            &report.command,
            report.error_type.display_name(),
            &report.title,
            &report.message,
            &report.raw_error,
            report.suggested_fix.as_deref(),
            &context,
        )
        .ok();

        // Create report with log path for display
        let mut display_report = report;
        display_report.log_path = log_path;

        // Show error notification
        show_error_notification(&display_report);

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            show_banner();
            Ok(())
        }
        Some(cmd) => commands::execute(cmd),
    }
}

/// Determine which command was being run for error reporting
fn determine_command() -> String {
    // This is a simplified version - in real impl would need access to CLI args
    "openzed-git".to_string()
}

/// Extract clean error message from anyhow error
fn extract_clean_message(err: &str) -> String {
    // Remove anyhow context prefix if present
    let msg = if err.contains("context了一步") {
        // Already formatted, just use as-is
        err.to_string()
    } else {
        err.to_string()
    };

    // Truncate if too long
    if msg.len() > 200 {
        msg[..200].to_string() + "..."
    } else {
        msg
    }
}
