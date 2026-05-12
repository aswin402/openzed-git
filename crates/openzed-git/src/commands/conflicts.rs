use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{ARROW, CHECK, CYAN, GREEN, PINK, PURPLE, TEXT, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_print, aura_text, error_msg, success, warning_msg};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Conflict Helper");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Get conflicted files
    let conflicted = GitInfo::conflicted_files().unwrap_or_default();

    if conflicted.is_empty() {
        // No conflicts
        success("No merge conflicts found.");
        println!();
        print!("  ");
        aura_text("If you're not in a merge, run ", TEXT_MUTED);
        aura_text("openzed-git rebase-helper", CYAN);
        aura_text(" to check rebase status.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Show header with Git state
    print!("  ");
    aura_text("Git state: ", TEXT_MUTED);
    aura_text("MERGE_CONFLICT", PURPLE);
    println!();

    println!();
    print!("  ");
    aura_text("Conflicted files:", TEXT_MUTED);
    println!();

    for file in &conflicted {
        print!("  ");
        aura_print(ARROW.trim(), PINK);
        aura_text(file, CYAN);
        println!();
    }

    println!();

    // Show menu
    let choices = [
        "Open conflicted files in Zed",
        "Show resolution steps",
        "Mark selected files as resolved",
        "Continue merge",
        "Abort merge",
        "Cancel",
    ];

    let selection = dialoguer::Select::new()
        .with_prompt("What do you want to do?")
        .items(&choices)
        .default(0)
        .interact()?;

    match selection {
        0 => open_conflicted_files(&conflicted)?,
        1 => show_resolution_steps(),
        2 => mark_resolved(&conflicted)?,
        3 => continue_merge()?,
        4 => abort_merge()?,
        5 => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
        }
        _ => {}
    }

    Ok(())
}

fn open_conflicted_files(files: &[String]) -> Result<()> {
    println!();

    if !GitInfo::zed_available() {
        warning_msg("Zed CLI not found. Open these files manually:");
        println!();
        for file in files {
            print!("  ");
            aura_text(file, CYAN);
            println!();
        }
        return Ok(());
    }

    print!("  Opening ");
    aura_text(&format!("{} file(s)", files.len()), CYAN);
    print!(" in Zed... ");

    match GitInfo::open_in_zed(files) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Files opened in Zed.");
            println!();
            print!("  ");
            aura_text("Resolve conflicts and mark files as resolved.", TEXT_MUTED);
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to open files: {}", e));
        }
    }

    Ok(())
}

fn show_resolution_steps() {
    println!();
    aura_text("Conflict Resolution Steps:", CYAN);
    println!();
    println!("  1. Open each conflicted file in Zed.");
    println!("  2. Search for conflict markers:");
    println!("     - <<<<<<< HEAD (your changes)");
    println!("     - ======= (separator)");
    println!("     - >>>>>>> (incoming changes)");
    println!("  3. Keep the correct code and remove markers.");
    println!("  4. Save the file.");
    println!("  5. Mark the file as resolved using this tool.");
    println!("  6. Continue merge when all conflicts are resolved.");
    println!();
    print!("  ");
    aura_text("Tip: ", TEXT_MUTED);
    aura_text("Use ", TEXT);
    aura_text("git diff", CYAN);
    aura_text(" to see all changes.", TEXT);
    println!();
}

fn mark_resolved(all_files: &[String]) -> Result<()> {
    println!();
    aura_text("Select files to mark as resolved:", CYAN);
    println!();

    if all_files.is_empty() {
        warning_msg("No conflicted files.");
        return Ok(());
    }

    let selections = dialoguer::MultiSelect::new()
        .with_prompt("Select files (space to toggle, enter to confirm)")
        .items(all_files)
        .interact()?;

    if selections.is_empty() {
        print!("  ");
        aura_text("No files selected.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    let selected: Vec<String> = selections.iter().map(|&i| all_files[i].clone()).collect();

    println!();
    print!("  Marking ");
    aura_text(&format!("{} file(s)", selected.len()), CYAN);
    print!(" as resolved... ");

    match GitInfo::mark_resolved(&selected) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Files marked as resolved.");
            println!();
            print!("  ");
            aura_text("Run ", TEXT_MUTED);
            aura_text("Continue merge", CYAN);
            aura_text(" when all conflicts are resolved.", TEXT_MUTED);
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to mark files: {}", e));
        }
    }

    Ok(())
}

fn continue_merge() -> Result<()> {
    println!();

    if !GitInfo::has_merge_in_progress() {
        warning_msg("No merge in progress.");
        print!("  ");
        aura_text("Use ", TEXT_MUTED);
        aura_text("git merge --continue", CYAN);
        aura_text(" manually if needed.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Check for remaining conflicts
    let conflicted = GitInfo::conflicted_files().unwrap_or_default();
    if !conflicted.is_empty() {
        warning_msg(&format!(
            "{} file(s) still have conflicts:",
            conflicted.len()
        ));
        for file in &conflicted {
            print!("  ");
            aura_text(file, CYAN);
            println!();
        }
        println!();
        print!("  ");
        aura_text("Resolve all conflicts before continuing.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Continuing merge... ");

    match GitInfo::continue_merge() {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Merge completed successfully.");
        }
        Err(e) => {
            println!();
            error_msg(&format!(
                "Merge failed: {}. Git may have opened an editor.",
                e
            ));
            println!();
            print!("  ");
            aura_text(
                "Check git status and resolve any remaining issues.",
                TEXT_MUTED,
            );
            println!();
        }
    }

    Ok(())
}

fn abort_merge() -> Result<()> {
    println!();
    print!("  ");
    aura_text(
        "This will abort the merge and return to previous state.",
        TEXT_MUTED,
    );
    println!();
    println!();

    let confirm_abort = confirm("Abort merge and return to previous state?")?;
    if !confirm_abort {
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    println!();
    print!("  Aborting merge... ");

    match GitInfo::abort_merge() {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Merge aborted successfully.");
            println!();
            print!("  ");
            aura_text(
                "Your changes are preserved in the working directory.",
                TEXT_MUTED,
            );
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to abort merge: {}", e));
        }
    }

    Ok(())
}
