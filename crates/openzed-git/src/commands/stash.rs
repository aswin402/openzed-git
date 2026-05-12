use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{CHECK, CYAN, GREEN, PURPLE, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use crate::ui::prompts::input_with_default;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Stash");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Check for changes
    let has_changes = GitInfo::has_changes().unwrap_or(false);

    if !has_changes {
        warning_msg("No changes to stash.");
        println!();
        println!("  ");
        aura_text("Use ", TEXT_MUTED);
        aura_text("git stash list", CYAN);
        aura_text(" to see saved stashes.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Show stash menu
    println!("  ");
    aura_text("Current changes detected.", CYAN);
    println!();

    let choices = [
        "Save stash",
        "List stashes",
        "Apply stash",
        "Pop latest stash",
        "Drop stash",
        "Cancel",
    ];

    let selection = dialoguer::Select::new()
        .with_prompt("What do you want to do?")
        .items(&choices)
        .default(0)
        .interact()?;

    match selection {
        0 => save_stash()?,
        1 => list_stashes()?,
        2 => apply_stash()?,
        3 => pop_stash()?,
        4 => drop_stash()?,
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

fn save_stash() -> Result<()> {
    println!();

    // Ask for stash message
    let message: String = input_with_default("Stash message:", "work in progress")?;

    print!("  Saving stash... ");

    match GitInfo::stash_push(&message) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Stash saved successfully.");
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to save stash: {}", e));
        }
    }

    Ok(())
}

fn list_stashes() -> Result<()> {
    println!();

    let stashes = GitInfo::stash_list()?;

    if stashes.is_empty() {
        warning_msg("No stashes found.");
        println!();
        print!("  ");
        aura_text("Use ", TEXT_MUTED);
        aura_text("openzed-git stash", CYAN);
        aura_text(" to save current changes.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    section("Stashes:");

    for stash in &stashes {
        print!("  ");
        // Stash index in purple
        aura_text(&format!("stash@{{{}}}", stash.index), PURPLE);
        print!(" ");
        // Branch in cyan
        aura_text(&format!("({})", stash.branch), CYAN);
        print!(" ");
        // Message in text
        aura_text(&stash.message, crate::ui::aura::aura::TEXT);
        println!();
    }

    println!();
    success(&format!("{} stash(es) found.", stashes.len()));

    Ok(())
}

fn apply_stash() -> Result<()> {
    println!();

    let stashes = GitInfo::stash_list()?;

    if stashes.is_empty() {
        warning_msg("No stashes to apply.");
        return Ok(());
    }

    section("Select a stash to apply:");

    let stash_labels: Vec<String> = stashes
        .iter()
        .map(|s| format!("stash@{{{}}}: {} on {}", s.index, s.message, s.branch))
        .collect();

    let selection = dialoguer::Select::new()
        .with_prompt("Which stash?")
        .items(&stash_labels)
        .default(0)
        .interact()?;

    let stash = &stashes[selection];

    println!();
    print!("  Applying ");
    aura_text(&format!("stash@{{{}}}", stash.index), PURPLE);
    print!("... ");

    match GitInfo::stash_apply(stash.index) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Stash applied successfully.");
            println!();
            print!("  ");
            aura_text("Check git status for any conflicts.", TEXT_MUTED);
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to apply stash: {}", e));
            println!();
            print!("  ");
            aura_text("There may be conflicts. Resolve them manually.", TEXT_MUTED);
            println!();
        }
    }

    Ok(())
}

fn pop_stash() -> Result<()> {
    println!();

    let stashes = GitInfo::stash_list()?;

    if stashes.is_empty() {
        warning_msg("No stashes to pop.");
        return Ok(());
    }

    let latest = &stashes[0];
    println!();
    print!("  Latest stash: ");
    aura_text(&format!("stash@{{{}}}", latest.index), PURPLE);
    print!(" - ");
    aura_text(&latest.message, crate::ui::aura::aura::TEXT);
    println!();

    // Confirm pop
    let confirm_pop = confirm("Pop this stash?")?;
    if !confirm_pop {
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Popping stash... ");

    match GitInfo::stash_pop(latest.index) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Stash popped successfully.");
            println!();
            print!("  ");
            aura_text("Check git status for any conflicts.", TEXT_MUTED);
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to pop stash: {}", e));
            println!();
            print!("  ");
            aura_text("There may be conflicts. Resolve them manually.", TEXT_MUTED);
            println!();
        }
    }

    Ok(())
}

fn drop_stash() -> Result<()> {
    println!();

    let stashes = GitInfo::stash_list()?;

    if stashes.is_empty() {
        warning_msg("No stashes to drop.");
        return Ok(());
    }

    section("Select a stash to drop:");

    let stash_labels: Vec<String> = stashes
        .iter()
        .map(|s| format!("stash@{{{}}}: {} on {}", s.index, s.message, s.branch))
        .collect();

    let selection = dialoguer::Select::new()
        .with_prompt("Which stash?")
        .items(&stash_labels)
        .default(0)
        .interact()?;

    let stash = &stashes[selection];

    // Confirm drop
    println!();
    print!("  ");
    aura_text("You are about to drop: ", TEXT_MUTED);
    aura_text(&format!("stash@{{{}}}", stash.index), PURPLE);
    println!();

    let confirm_drop = confirm("Are you sure? This cannot be undone.")?;
    if !confirm_drop {
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Dropping stash... ");

    match GitInfo::stash_drop(stash.index) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Stash dropped successfully.");
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to drop stash: {}", e));
        }
    }

    Ok(())
}

fn section(title: &str) {
    print!("  ");
    aura_text(title, CYAN);
    println!();
}
