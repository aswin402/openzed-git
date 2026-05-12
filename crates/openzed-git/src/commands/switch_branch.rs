use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{CHECK, CYAN, GREEN, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use crate::ui::prompts::input;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Branch Switcher");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Get current branch
    let current_branch = match GitInfo::current_branch() {
        Ok(b) => b,
        Err(_) => String::from("(detached)"),
    };

    // Show current branch
    println!("  ");
    aura_text("Current branch: ", TEXT_MUTED);
    aura_text(&current_branch, CYAN);
    println!();

    // Check for uncommitted changes
    let has_changes = GitInfo::has_changes().unwrap_or(false);
    if has_changes {
        println!();
        warning_msg("You have uncommitted changes.");
        println!("  ");
        aura_text(
            "Switching branches with uncommitted changes may fail if there are conflicts.",
            TEXT_MUTED,
        );
        println!();
    }

    // Main menu
    println!();
    let choices = [
        "Switch branch",
        "Create new branch",
        "Checkout remote branch",
        "Cancel",
    ];

    let selection = dialoguer::Select::new()
        .with_prompt("What do you want to do?")
        .items(&choices)
        .default(0)
        .interact()?;

    match selection {
        0 => switch_branch(&current_branch, has_changes)?,
        1 => create_branch(&current_branch, has_changes)?,
        2 => checkout_remote(&current_branch, has_changes)?,
        3 => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
        }
        _ => {}
    }

    Ok(())
}

fn switch_branch(current: &str, has_changes: bool) -> Result<()> {
    println!();

    let branches = GitInfo::local_branches()?;
    if branches.is_empty() {
        warning_msg("No local branches found.");
        return Ok(());
    }

    // Mark current branch
    let branch_labels: Vec<String> = branches
        .iter()
        .map(|b| {
            if b == current {
                format!("* {}", b)
            } else {
                b.clone()
            }
        })
        .collect();

    let selection = dialoguer::Select::new()
        .with_prompt("Select a branch to switch to:")
        .items(&branch_labels)
        .default(0)
        .interact()?;

    let selected = &branches[selection];

    if selected == current {
        println!();
        warning_msg(&format!("Already on branch {}.", current));
        return Ok(());
    }

    // Confirm if there are changes
    if has_changes {
        println!();
        let continue_switch =
            confirm("Continue switching branch? This may fail if changes conflict.")?;
        if !continue_switch {
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }
    }

    print!("  Switching to {}... ", selected);

    match GitInfo::switch_branch(selected) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success(&format!("Switched to branch {}.", selected));
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to switch branch: {}", e));
        }
    }

    Ok(())
}

fn create_branch(_current: &str, has_changes: bool) -> Result<()> {
    println!();

    // Confirm if there are changes
    if has_changes {
        println!();
        let continue_create =
            confirm("Continue creating a new branch? This may fail if changes conflict.")?;
        if !continue_create {
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }
    }

    // Get new branch name
    let new_branch = match input("New branch name:") {
        Ok(name) => name.trim().to_string(),
        Err(_) => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }
    };

    // Validate branch name
    if new_branch.is_empty() {
        error_msg("Branch name cannot be empty.");
        return Ok(());
    }

    if new_branch.contains(' ') {
        error_msg("Branch name cannot contain spaces.");
        return Ok(());
    }

    // Check if branch already exists
    if GitInfo::branch_exists(&new_branch)? {
        error_msg(&format!("Branch '{}' already exists.", new_branch));
        return Ok(());
    }

    print!("  Creating and switching to {}... ", new_branch);

    match GitInfo::create_and_switch_branch(&new_branch) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success(&format!("Created and switched to branch {}.", new_branch));
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to create branch: {}", e));
        }
    }

    Ok(())
}

fn checkout_remote(_current: &str, has_changes: bool) -> Result<()> {
    println!();

    // Confirm if there are changes
    if has_changes {
        println!();
        let continue_checkout =
            confirm("Continue checking out remote branch? This may fail if changes conflict.")?;
        if !continue_checkout {
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }
    }

    let remote_branches = GitInfo::remote_branches()?;
    if remote_branches.is_empty() {
        warning_msg("No remote branches found.");
        return Ok(());
    }

    // Show remote branches in purple
    let branch_labels: Vec<String> = remote_branches
        .iter()
        .map(|b| {
            // Extract local name for display
            let local_name = b.split('/').skip(1).collect::<Vec<_>>().join("/");
            if local_name.is_empty() {
                b.clone()
            } else {
                format!("{} -> {}", b, local_name)
            }
        })
        .collect();

    let selection = dialoguer::Select::new()
        .with_prompt("Select a remote branch to checkout:")
        .items(&branch_labels)
        .default(0)
        .interact()?;

    let selected = &remote_branches[selection];

    // Extract local branch name
    let local_name: String = selected.split('/').skip(1).collect::<Vec<_>>().join("/");

    // Check if local branch already exists
    if !local_name.is_empty() && GitInfo::branch_exists(&local_name)? {
        println!();
        aura_text("Local branch '", TEXT_MUTED);
        aura_text(&local_name, CYAN);
        aura_text("' already exists.", TEXT_MUTED);
        println!();

        let switch_existing = confirm(&format!(
            "Switch to existing local branch '{}'?",
            local_name
        ))?;
        if !switch_existing {
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }

        print!("  Switching to {}... ", local_name);
        match GitInfo::switch_branch(&local_name) {
            Ok(_) => {
                aura_text(CHECK.trim(), GREEN);
                println!();
                success(&format!("Switched to branch {}.", local_name));
            }
            Err(e) => {
                println!();
                error_msg(&format!("Failed to switch: {}", e));
            }
        }
    } else {
        print!("  Checking out {}... ", selected);

        match GitInfo::checkout_remote_branch(selected) {
            Ok(_) => {
                aura_text(CHECK.trim(), GREEN);
                println!();
                if local_name.is_empty() {
                    success(&format!("Checked out {}.", selected));
                } else {
                    success(&format!(
                        "Checked out {} as branch {}.",
                        selected, local_name
                    ));
                }
            }
            Err(e) => {
                println!();
                error_msg(&format!("Failed to checkout: {}", e));
            }
        }
    }

    Ok(())
}
