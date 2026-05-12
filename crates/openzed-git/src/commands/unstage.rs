use crate::core::git::GitInfo;
use crate::ui::aura::aura::{CHECK, CYAN, GREEN, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Unstage Files");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Get staged files
    let files = match GitInfo::staged_files() {
        Ok(f) => f,
        Err(e) => {
            error_msg(&format!("Failed to get staged files: {}", e));
            return Ok(());
        }
    };

    if files.is_empty() {
        warning_msg("No staged files to unstage.");
        return Ok(());
    }

    // Show staged files
    println!("  ");
    aura_text("Staged files:", TEXT_MUTED);
    println!();

    let selections = dialoguer::MultiSelect::new()
        .with_prompt("Select files to unstage (space to select, enter to confirm)")
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

    // Unstage files
    print!("  Unstaging files... ");

    match GitInfo::unstage_files(&selected_files) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success(&format!("Unstaged {} file(s).", selected_files.len()));
            println!();
            print!("  ");
            aura_text("Changes are still in your working directory.", TEXT_MUTED);
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to unstage files: {}", e));
        }
    }

    Ok(())
}
