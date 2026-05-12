use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{CHECK, CYAN, GREEN, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, success};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Undo Last Commit");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Check if there are commits
    if !GitInfo::has_commits().unwrap_or(false) {
        error_msg("No commits to undo.");
        return Ok(());
    }

    // Get last commit info
    let commit = match GitInfo::last_commit_summary() {
        Ok(c) => c,
        Err(e) => {
            error_msg(&format!("Failed to get last commit: {}", e));
            return Ok(());
        }
    };

    // Show last commit
    println!("  ");
    aura_text("Last commit:", TEXT_MUTED);
    println!();
    print!("  ");
    aura_text("› ", CYAN);
    aura_text(&commit.hash, CYAN);
    print!("  ");
    aura_text(&commit.message, crate::ui::aura::aura::TEXT);
    if !commit.date.is_empty() {
        print!("  ");
        aura_text(&commit.date, TEXT_MUTED);
    }
    println!();

    // Check if this is the only commit
    let _branch = GitInfo::current_branch().unwrap_or_default();
    println!();

    // Ask for undo type
    let choices = [
        "Soft reset - keep changes staged",
        "Mixed reset - keep changes unstaged",
        "Cancel",
    ];

    let selection = dialoguer::Select::new()
        .with_prompt("Choose undo type:")
        .items(&choices)
        .default(0)
        .interact()?;

    match selection {
        0 => soft_reset()?,
        1 => mixed_reset()?,
        2 => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
        }
        _ => {}
    }

    Ok(())
}

fn soft_reset() -> Result<()> {
    println!();
    let confirm_reset = confirm("Run: git reset --soft HEAD~1")?;
    if !confirm_reset {
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Undoing last commit (soft)... ");

    match GitInfo::undo_last_commit_soft() {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Last commit undone. Changes are still staged.");
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to undo commit: {}", e));
        }
    }

    Ok(())
}

fn mixed_reset() -> Result<()> {
    println!();
    let confirm_reset = confirm("Run: git reset --mixed HEAD~1")?;
    if !confirm_reset {
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Undoing last commit (mixed)... ");

    match GitInfo::undo_last_commit_mixed() {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Last commit undone. Changes are now unstaged.");
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to undo commit: {}", e));
        }
    }

    Ok(())
}
