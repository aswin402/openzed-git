use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::ui::aura::aura::{CYAN, GREEN, PURPLE, TEXT_MUTED, YELLOW};
use crate::ui::aura::separator;
use crate::core::shell::confirm;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use crate::ui::prompts::select_with_default;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Pull Requests");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        return Ok(());
    }

    // Check GitHub CLI
    if let Err(e) = GithubInfo::ensure_gh_ready() {
        error_msg("GitHub CLI not ready.");
        aura_text(&format!(" {}", e), TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Get PRs
    let prs = match GithubInfo::pr_list() {
        Ok(list) => list,
        Err(e) => {
            error_msg("Failed to list PRs.");
            aura_text(&format!(" {}", e), TEXT_MUTED);
            println!();
            return Ok(());
        }
    };

    if prs.is_empty() {
        warning_msg("No open pull requests found.");
        return Ok(());
    }

    // Display PRs
    println!();
    aura_text(&format!("{} open pull requests:", prs.len()), TEXT_MUTED);
    println!();

    for pr in &prs {
        print!("  ");
        aura_text(&format!("#{}", pr.number), PURPLE);
        print!("  ");
        aura_text(&pr.head, CYAN);
        print!("  ");
        aura_text("→", crate::ui::aura::aura::PINK);
        print!("  ");
        aura_text(&pr.base, CYAN);
        print!("  ");
        aura_text(&pr.title, crate::ui::aura::aura::TEXT);
        println!();

        print!("      ");
        if pr.is_draft {
            aura_text("DRAFT", YELLOW);
        } else {
            aura_text(&pr.state, GREEN);
        }
        print!("  ");
        aura_text(&format!("updated {}", pr.updated_at), TEXT_MUTED);
        print!("  ");
        aura_text(&format!("by {}", pr.author), TEXT_MUTED);
        println!();
    }

    // Action menu
    println!();
    let selection = select_with_default(
        "What do you want to do?",
        &[
            "Open PR in browser",
            "Checkout PR",
            "Refresh list",
            "Cancel",
        ],
        0,
    )?;

    match selection {
        0 => {
            // Open in browser
            let pr_labels: Vec<String> = prs
                .iter()
                .map(|pr| format!("#{} {}", pr.number, pr.title))
                .collect();

            let pr_selection = select_with_default(
                "Select PR to open:",
                &pr_labels.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                0,
            )?;

            let selected = &prs[pr_selection];

            print!("  Opening in browser... ");
            match GithubInfo::pr_view_web(selected.number) {
                Ok(_) => {
                    aura_text(crate::ui::aura::aura::CHECK.trim(), GREEN);
                    println!();
                    success("Opened PR in browser.");
                }
                Err(e) => {
                    println!();
                    error_msg(&format!("Failed to open PR: {}", e));
                }
            }
        }
        1 => {
            // Checkout PR
            let pr_labels: Vec<String> = prs
                .iter()
                .map(|pr| format!("#{} {}", pr.number, pr.title))
                .collect();

            let pr_selection = select_with_default(
                "Select PR to checkout:",
                &pr_labels.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                0,
            )?;

            let selected = &prs[pr_selection];

            // Warn about uncommitted changes
            let has_changes = GitInfo::has_changes().unwrap_or(false);
            if has_changes {
                println!();
                warning_msg("You have uncommitted changes.");
                let continue_checkout = confirm("Continue checkout?")?;
                if !continue_checkout {
                    println!();
                    print!("  ");
                    aura_text("Cancelled.", TEXT_MUTED);
                    println!();
                    return Ok(());
                }
            }

            print!("  Checking out PR #{}... ", selected.number);
            match GithubInfo::pr_checkout(selected.number) {
                Ok(_) => {
                    aura_text(crate::ui::aura::aura::CHECK.trim(), GREEN);
                    println!();
                    success(&format!("Checked out PR #{}", selected.number));
                }
                Err(e) => {
                    println!();
                    error_msg(&format!("Failed to checkout PR: {}", e));
                }
            }
        }
        2 => {
            // Refresh - just re-run
            println!();
            return run();
        }
        _ => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
        }
    }

    Ok(())
}


