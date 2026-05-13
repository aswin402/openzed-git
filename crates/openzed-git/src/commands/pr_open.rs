use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::ui::aura::aura::{CHECK, GREEN, PURPLE, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::core::shell::confirm;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use crate::ui::prompts::select_with_default;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Open Pull Request");

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

    // Try to get current PR
    match GithubInfo::current_pr_url() {
        Ok(Some(url)) => {
            // Current branch has an associated PR
            println!();
            aura_text("Current branch has a PR:", TEXT_MUTED);
            println!();
            aura_text(&url, PURPLE);
            println!();

            if !confirm("Open in browser?")? {
                println!();
                print!("  ");
                aura_text("Cancelled.", TEXT_MUTED);
                println!();
                return Ok(());
            }

            print!("  Opening in browser... ");
            match GithubInfo::pr_view_web_current() {
                Ok(_) => {
                    aura_text(CHECK.trim(), GREEN);
                    println!();
                    success("Opened PR in browser.");
                }
                Err(e) => {
                    println!();
                    error_msg(&format!("Failed to open: {}", e));
                }
            }
        }
        Ok(None) => {
            // No PR associated with current branch
            println!();
            warning_msg("No PR found for current branch.");
            println!();

            // List PRs to choose from
            list_and_open_prs()?;
        }
        Err(e) => {
            println!();
            error_msg("Failed to check current PR.");
            aura_text(&format!(" {}", e), TEXT_MUTED);
            println!();
            list_and_open_prs()?;
        }
    }

    Ok(())
}

fn list_and_open_prs() -> Result<()> {
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

    println!();
    aura_text("Open PRs:", TEXT_MUTED);
    println!();

    let pr_labels: Vec<String> = prs
        .iter()
        .map(|pr| format!("#{}  {}  →  {}", pr.number, pr.title, pr.base))
        .collect();

    let selection = select_with_default(
        "Select PR to open:",
        &pr_labels.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        0,
    )?;

    let selected = &prs[selection];

    print!("  Opening PR #{} in browser... ", selected.number);

    match GithubInfo::pr_view_web(selected.number) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success(&format!(
                "Opened PR #{}: {}",
                selected.number, selected.title
            ));
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to open PR: {}", e));
        }
    }

    Ok(())
}


