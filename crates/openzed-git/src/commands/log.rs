#![allow(unused)]

use crate::core::paths;
use crate::ui::aura::aura::{ERROR, TEXT, WARNING};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Get project error log path
fn error_log_path() -> Option<PathBuf> {
    let log_dir = paths::project_log_dir();
    // Only return if we're in a git repo (project level)
    if std::path::Path::new(".git").exists() {
        Some(log_dir.join("gitlogerror.md"))
    } else {
        None
    }
}

/// Get content of a log file
fn read_log(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

/// Print colored section header
fn print_header(title: &str, color: &str) {
    println!("\n{}", color);
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  {} ", title);
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!("{}", TEXT);
}

pub fn run() -> Result<()> {
    println!("{}", TEXT);

    let log_path = error_log_path();

    match log_path {
        Some(path) if path.exists() => {
            print_header("OpenZed Git - Error Log", WARNING);

            match read_log(&path) {
                Ok(content) => {
                    // Skip the header lines and show content
                    let lines: Vec<&str> = content.lines().collect();
                    if lines.len() > 2 {
                        for line in lines.iter().skip(2) {
                            println!("{}", line);
                        }
                    } else {
                        println!("  (empty log)");
                    }
                }
                Err(e) => {
                    println!("{} Failed to read log: {}", ERROR, e);
                }
            }

            println!("\n{} Log file: {}", TEXT, path.display());
        }
        _ => {
            println!("{} No error logs found.", WARNING);
            println!("{} Error logs are created automatically when commands fail.", TEXT);
            println!("\nLog location:");
            println!("  • .openzed-git/gitlogerror.md");
        }
    }

    Ok(())
}