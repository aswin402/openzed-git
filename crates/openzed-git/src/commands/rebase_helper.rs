use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{ARROW, CHECK, CYAN, GREEN, PINK, PURPLE, TEXT, TEXT_MUTED, YELLOW};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_print, aura_text, error_msg, success, warning_msg};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Rebase Helper");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Check if rebase is in progress
    if GitInfo::has_rebase_in_progress() {
        show_rebase_in_progress_menu()
    } else {
        show_no_rebase_menu()
    }
}

fn show_rebase_in_progress_menu() -> Result<()> {
    // Get conflicted files if any
    let conflicted = GitInfo::conflicted_files().unwrap_or_default();

    // Show header with Git state
    print!("  ");
    aura_text("Git state: ", TEXT_MUTED);
    aura_text("REBASE_IN_PROGRESS", PURPLE);
    println!();

    if !conflicted.is_empty() {
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
    }

    println!();

    // Show menu
    let choices = [
        "Open conflicted files in Zed",
        "Mark selected files as resolved",
        "Continue rebase",
        "Skip current commit",
        "Abort rebase",
        "Show rebase steps",
        "Cancel",
    ];

    let selection = dialoguer::Select::new()
        .with_prompt("What do you want to do?")
        .items(&choices)
        .default(0)
        .interact()?;

    match selection {
        0 => open_conflicted_files(&conflicted)?,
        1 => mark_resolved(&conflicted)?,
        2 => continue_rebase()?,
        3 => skip_rebase()?,
        4 => abort_rebase()?,
        5 => show_rebase_steps(),
        6 => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
        }
        _ => {}
    }

    Ok(())
}

fn show_no_rebase_menu() -> Result<()> {
    warning_msg("No rebase in progress.");

    println!();
    print!("  ");
    aura_text("Use ", TEXT_MUTED);
    aura_text("openzed-git conflicts", CYAN);
    aura_text(" if you're in a merge conflict.", TEXT_MUTED);
    println!();

    Ok(())
}

fn open_conflicted_files(files: &[String]) -> Result<()> {
    println!();

    if files.is_empty() {
        success("No conflicted files.");
        return Ok(());
    }

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
            aura_text("Continue rebase", CYAN);
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

fn continue_rebase() -> Result<()> {
    println!();

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

    print!("  Continuing rebase... ");

    match GitInfo::continue_rebase() {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Rebase continued successfully.");
        }
        Err(e) => {
            println!();
            error_msg(&format!(
                "Rebase failed: {}. Git may have opened an editor.",
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

fn skip_rebase() -> Result<()> {
    println!();
    print!("  ");
    aura_text("WARNING: ", TEXT_MUTED);
    aura_text(
        "Skipping a commit discards its changes permanently.",
        YELLOW,
    );
    println!();
    println!();
    print!("  ");
    aura_text("This cannot be undone. Are you sure?", TEXT_MUTED);
    println!();
    println!();

    let confirm_skip = confirm("Skip current commit during rebase?")?;
    if !confirm_skip {
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    println!();
    print!("  Skipping commit... ");

    match GitInfo::skip_rebase() {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Commit skipped.");
            println!();
            print!("  ");
            aura_text("Run ", TEXT_MUTED);
            aura_text("Continue rebase", CYAN);
            aura_text(" to proceed with the next commit.", TEXT_MUTED);
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to skip commit: {}", e));
        }
    }

    Ok(())
}

fn abort_rebase() -> Result<()> {
    println!();
    print!("  ");
    aura_text("WARNING: ", TEXT_MUTED);
    aura_text(
        "Aborting rebase will discard all changes made during the rebase.",
        YELLOW,
    );
    println!();
    println!();
    print!("  ");
    aura_text(
        "Your original commits are preserved in reflog for recovery.",
        TEXT_MUTED,
    );
    println!();
    println!();

    let confirm_abort = confirm("Abort rebase and return to original state?")?;
    if !confirm_abort {
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    println!();
    print!("  Aborting rebase... ");

    match GitInfo::abort_rebase() {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Rebase aborted successfully.");
            println!();
            print!("  ");
            aura_text(
                "Your branch has been reset to its original state.",
                TEXT_MUTED,
            );
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to abort rebase: {}", e));
        }
    }

    Ok(())
}

fn show_rebase_steps() {
    println!();
    aura_text("Rebase Steps:", CYAN);
    println!();
    println!("  1. Resolve any conflict markers in the conflicted files.");
    println!("  2. Stage the resolved files using \"Mark as resolved\".");
    println!("  3. Continue the rebase.");
    println!("  4. Repeat until the rebase completes.");
    println!();
    print!("  ");
    aura_text("Tip: ", TEXT_MUTED);
    aura_text("Use ", TEXT);
    aura_text("git log --oneline", CYAN);
    aura_text(" to see the rebase plan.", TEXT);
    println!();
}
