use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::{colors::*, output::separator, prompts::input};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Rename Branch");

    let info = GitInfo::get()?;
    let current = info
        .current_branch
        .clone()
        .unwrap_or_else(|| "main".to_string());

    println!("  Current branch: {}", current);

    let new_name: String = input("New branch name:")?;

    if new_name.trim().is_empty() {
        println!("  Cancelled.");
        return Ok(());
    }

    // Confirm
    println!();
    let confirm_rename = confirm(&format!("Run: git branch -m {} {}", current, new_name))?;

    if !confirm_rename {
        println!("  Cancelled.");
        return Ok(());
    }

    GitInfo::branch_rename(&current, &new_name)?;
    println!("  {} Branch renamed to '{}'", CHECK, new_name);

    // If remote exists, ask about pushing
    if info.remote_origin.is_some() {
        println!();
        let push = confirm("Push renamed branch and set upstream?")?;
        if push {
            GitInfo::push_u("origin", &new_name)?;
            println!("  {} Pushed renamed branch", CHECK);
        }
    }

    Ok(())
}
