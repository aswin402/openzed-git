use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::ui::aura::aura::{CHECK, CYAN, GREEN, PURPLE, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::core::shell::confirm;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use crate::ui::prompts::select_with_default;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Checkout Pull Request");

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
    aura_text("Select a PR to checkout:", TEXT_MUTED);
    println!();

    let pr_labels: Vec<String> = prs
        .iter()
        .map(|pr| format!("#{}  {}  →  {}", pr.number, pr.title, pr.base))
        .collect();

    let selection = select_with_default(
        "Which PR?",
        &pr_labels.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        0,
    )?;

    let selected = &prs[selection];

    // Show selected PR
    println!();
    aura_text("Selected: ", TEXT_MUTED);
    aura_text(&format!("#{}", selected.number), PURPLE);
    print!("  ");
    aura_text(&selected.title, crate::ui::aura::aura::TEXT);
    println!();

    // Warn about uncommitted changes
    let has_changes = GitInfo::has_changes().unwrap_or(false);
    if has_changes {
        println!();
        warning_msg("You have uncommitted changes.");
    }

    // Confirm
    println!();
    if !confirm("Checkout this PR?")? {
        println!();
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Checkout
    print!("  Checking out PR #{}... ", selected.number);

    match GithubInfo::pr_checkout(selected.number) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success(&format!(
                "Checked out PR #{}: {}",
                selected.number, selected.title
            ));
            println!();
            aura_text("Branch: ", TEXT_MUTED);
            aura_text(&selected.head, CYAN);
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to checkout PR: {}", e));
        }
    }

    Ok(())
}


