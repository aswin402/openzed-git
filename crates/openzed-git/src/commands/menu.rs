use crate::core::config::load_config;
use crate::core::git::GitInfo;
use crate::ui::aura::aura::{CYAN, TEXT, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Load config to get theme
    let _config = load_config().unwrap_or_default();

    loop {
        println!();
        aura_text("What do you want to do?", TEXT);
        println!();

        let choices = [
            "Status+",
            "Git Graph",
            "Commit",
            "Commit + Push",
            "Stash",
            "Switch Branch",
            "Pull",
            "Push",
            "Pull Requests",
            "Conflict Helper",
            "Rebase Helper",
            "Config",
            "Theme",
            "Doctor",
            "Cancel",
        ];

        let selection = dialoguer::Select::new()
            .with_prompt("Select an option")
            .items(&choices)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                println!();
                if let Err(e) = crate::commands::status_plus::run() {
                    error_msg(&format!("Status+ failed: {}", e));
                }
            }
            1 => {
                println!();
                if let Err(e) = crate::commands::graph::run() {
                    error_msg(&format!("Git Graph failed: {}", e));
                }
            }
            2 => {
                println!();
                if let Err(e) = crate::commands::commit::run() {
                    error_msg(&format!("Commit failed: {}", e));
                }
            }
            3 => {
                println!();
                if let Err(e) = crate::commands::commit_push::run() {
                    error_msg(&format!("Commit + Push failed: {}", e));
                }
            }
            4 => {
                println!();
                if let Err(e) = crate::commands::stash::run() {
                    error_msg(&format!("Stash failed: {}", e));
                }
            }
            5 => {
                println!();
                if let Err(e) = crate::commands::switch_branch::run() {
                    error_msg(&format!("Switch Branch failed: {}", e));
                }
            }
            6 => {
                println!();
                if let Err(e) = crate::commands::pull::run() {
                    error_msg(&format!("Pull failed: {}", e));
                }
            }
            7 => {
                println!();
                if let Err(e) = crate::commands::push::run() {
                    error_msg(&format!("Push failed: {}", e));
                }
            }
            8 => {
                println!();
                if let Err(e) = pull_requests_menu() {
                    error_msg(&format!("Pull Requests menu failed: {}", e));
                }
            }
            9 => {
                println!();
                if let Err(e) = crate::commands::conflicts::run() {
                    error_msg(&format!("Conflict Helper failed: {}", e));
                }
            }
            10 => {
                println!();
                if let Err(e) = crate::commands::rebase_helper::run() {
                    error_msg(&format!("Rebase Helper failed: {}", e));
                }
            }
            11 => {
                println!();
                if let Err(e) = crate::commands::config::run() {
                    error_msg(&format!("Config failed: {}", e));
                }
            }
            12 => {
                println!();
                if let Err(e) = crate::commands::theme::run() {
                    error_msg(&format!("Theme failed: {}", e));
                }
            }
            13 => {
                println!();
                if let Err(e) = crate::commands::doctor::run() {
                    error_msg(&format!("Doctor failed: {}", e));
                }
            }
            14 => {
                println!();
                print!("  ");
                aura_text("Goodbye!", TEXT_MUTED);
                println!();
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn pull_requests_menu() -> Result<()> {
    loop {
        println!();
        aura_text("Pull Requests", CYAN);
        println!();

        let choices = ["Create PR", "List PRs", "Checkout PR", "Open PR", "Back"];

        let selection = dialoguer::Select::new()
            .with_prompt("Select an option")
            .items(&choices)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                println!();
                if let Err(e) = crate::commands::pr_create::run() {
                    error_msg(&format!("Create PR failed: {}", e));
                }
            }
            1 => {
                println!();
                if let Err(e) = crate::commands::pr_list::run() {
                    error_msg(&format!("List PRs failed: {}", e));
                }
            }
            2 => {
                println!();
                if let Err(e) = crate::commands::pr_checkout::run() {
                    error_msg(&format!("Checkout PR failed: {}", e));
                }
            }
            3 => {
                println!();
                if let Err(e) = crate::commands::pr_open::run() {
                    error_msg(&format!("Open PR failed: {}", e));
                }
            }
            4 => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
