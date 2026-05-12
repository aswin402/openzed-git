use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{CHECK, CYAN, GREEN, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Restore Files");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Get modified unstaged files
    let files = match GitInfo::modified_unstaged_files() {
        Ok(f) => f,
        Err(e) => {
            error_msg(&format!("Failed to get modified files: {}", e));
            return Ok(());
        }
    };

    if files.is_empty() {
        warning_msg("No unstaged file changes to restore.");
        return Ok(());
    }

    // Show modified files
    println!("  ");
    aura_text("Modified files:", TEXT_MUTED);
    println!();

    let selections = dialoguer::MultiSelect::new()
        .with_prompt("Select files to restore (space to select, enter to confirm)")
        .items(&files)
        .interact()?;

    if selections.is_empty() {
        println!();
        print!("  ");
        aura_text("No files selected.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Get selected files
    let selected_files: Vec<String> = selections.iter().map(|&i| files[i].clone()).collect();

    println!();
    aura_text("Selected files:", TEXT_MUTED);
    for file in &selected_files {
        print!("  ");
        aura_text("› ", CYAN);
        aura_text(file, crate::ui::aura::aura::TEXT);
        println!();
    }

    // Warning before destructive action
    println!();
    warning_msg("This will discard local changes in selected files.");
    println!();

    // Confirm with explicit no as default
    let confirm_restore = confirm("Discard selected file changes?")?;
    if !confirm_restore {
        println!();
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Restore files
    print!("  Restoring files... ");

    match GitInfo::restore_files(&selected_files) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success(&format!("Restored {} file(s).", selected_files.len()));
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to restore files: {}", e));
        }
    }

    Ok(())
}
