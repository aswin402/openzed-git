use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{CHECK, CYAN, GREEN, PURPLE, TEXT, TEXT_MUTED, YELLOW};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use crate::ui::prompts::input;
use anyhow::Result;

const COMMIT_TYPES: &[(&str, &str)] = &[
    ("feat", "New feature"),
    ("fix", "Bug fix"),
    ("docs", "Documentation"),
    ("style", "Formatting/style only"),
    ("refactor", "Code change without feature/fix"),
    ("perf", "Performance improvement"),
    ("test", "Tests"),
    ("build", "Build system/dependencies"),
    ("ci", "CI/CD changes"),
    ("chore", "Maintenance"),
    ("revert", "Revert change"),
];

pub fn run() -> Result<()> {
    separator("OpenZed Git: Commit Assistant");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        print!("  ");
        aura_text("Run this command from within a Git repository.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Get changed files
    let changed = GitInfo::changed_files()?;
    if changed.is_empty() {
        warning_msg("No changes to commit.");
        return Ok(());
    }

    // Show changed files
    println!("  ");
    aura_text("Changed files:", TEXT_MUTED);
    println!();
    for file in &changed {
        print!("  ");
        aura_text("› ", CYAN);
        aura_text(file, TEXT);
        println!();
    }

    // Stage files menu
    println!();
    let staging_choice = dialoguer::Select::new()
        .with_prompt("What do you want to stage?")
        .items(&[
            "Select files",
            "Stage all files",
            "Use already staged files",
            "Cancel",
        ])
        .default(0)
        .interact()?;

    let staged_files: Vec<String>;

    match staging_choice {
        0 => {
            // Multi-select files
            println!();
            aura_text("Select files to stage:", TEXT_MUTED);
            println!();

            let selections = dialoguer::MultiSelect::new()
                .with_prompt("Select files (space to select, enter to confirm)")
                .items(&changed)
                .interact()?;

            if selections.is_empty() {
                println!();
                print!("  ");
                aura_text("No files selected.", TEXT_MUTED);
                println!();
                return Ok(());
            }

            staged_files = selections.iter().map(|&i| changed[i].clone()).collect();

            println!();
            aura_text("Staging files...", TEXT_MUTED);
            print!("  ");
            GitInfo::stage_files(&staged_files)?;
            aura_text(CHECK.trim(), GREEN);
            println!();
            success(&format!("Staged {} file(s).", staged_files.len()));
        }
        1 => {
            // Stage all
            println!();
            aura_text("Staging all files...", TEXT_MUTED);
            print!("  ");
            GitInfo::stage_all()?;
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Staged all files.");
            staged_files = GitInfo::staged_files()?;
        }
        2 => {
            // Use already staged
            staged_files = GitInfo::staged_files()?;
            if staged_files.is_empty() {
                println!();
                warning_msg("No staged files found. Please stage files first.");
                return Ok(());
            }
            println!();
            aura_text("Using already staged files:", TEXT_MUTED);
            for file in &staged_files {
                print!("  ");
                aura_text("› ", CYAN);
                aura_text(file, TEXT);
                println!();
            }
        }
        3 => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }
        _ => return Ok(()),
    }

    // Check if we have staged files
    if staged_files.is_empty() {
        println!();
        error_msg("No files staged. Cannot commit.");
        return Ok(());
    }

    // Choose commit type
    println!();
    aura_text("Commit type:", TEXT_MUTED);
    let type_labels: Vec<String> = COMMIT_TYPES
        .iter()
        .map(|(t, desc)| format!("{} - {}", t, desc))
        .collect();

    let type_selection = dialoguer::Select::new()
        .with_prompt("Select commit type:")
        .items(&type_labels)
        .default(0)
        .interact()?;

    let commit_type = COMMIT_TYPES[type_selection].0;

    // Ask for optional scope
    println!();
    let scope: String = input("Scope (optional):")
        .unwrap_or_default()
        .trim()
        .to_string();

    // Validate scope - reject spaces
    if scope.contains(' ') && !scope.is_empty() {
        error_msg("Scope cannot contain spaces. Use hyphens instead.");
        return Ok(());
    }

    // Ask for commit summary
    println!();
    let summary = match input("Commit summary:") {
        Ok(s) => s.trim().to_string(),
        Err(_) => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }
    };

    if summary.is_empty() {
        error_msg("Commit summary cannot be empty.");
        return Ok(());
    }

    // Check for ending period
    if summary.ends_with('.') {
        warning_msg("Commit summary should not end with a period.");
    }

    // Warn if too long
    if summary.len() > 72 {
        println!();
        aura_text("Warning: Summary is longer than 72 characters.", YELLOW);
        println!();
    }

    // Optional body
    println!();
    let add_body = confirm("Add commit body?")?;
    let body = if add_body {
        let body_input: String = input("Commit body:").unwrap_or_default().trim().to_string();
        if body_input.is_empty() {
            None
        } else {
            Some(body_input)
        }
    } else {
        None
    };

    // Build final commit message
    let commit_message = if scope.is_empty() {
        format!("{}: {}", commit_type, summary)
    } else {
        format!("{}({}): {}", commit_type, scope, summary)
    };

    // Preview
    println!();
    separator("OpenZed Git: Commit Preview");
    println!();
    print!("  ");
    aura_text(&commit_message, PURPLE);
    println!();

    if body.is_some() {
        println!();
        print!("  ");
        aura_text(body.as_ref().unwrap(), TEXT_MUTED);
        println!();
    }

    println!();
    let confirm_commit = confirm("Commit these changes?")?;
    if !confirm_commit {
        println!();
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Commit
    print!("  Creating commit... ");

    match GitInfo::create_commit(&commit_message, body.as_deref()) {
        Ok(_) => {
            aura_text(CHECK.trim(), GREEN);
            println!();

            // Get commit hash
            let hash = GitInfo::last_commit_hash().unwrap_or_default();
            println!();
            success("Commit created successfully.");
            print!("  ");
            aura_text("Commit: ", TEXT_MUTED);
            aura_text(&hash, CYAN);
            println!();
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to create commit: {}", e));
        }
    }

    Ok(())
}
