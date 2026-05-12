use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::separator;
use crate::ui::aura::aura::CHECK;
use crate::ui::prompts::input;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Setup Remote");

    // Check existing remote
    let info = GitInfo::get()?;

    if let Some(ref remote) = info.remote_origin {
        println!("  Remote origin already exists:");
        println!("    {}", remote);
        println!();
        println!("  Canceling. Use 'push' to update or 'rename-branch' to change.");
        return Ok(());
    }

    // Ask for URL
    let url: String = input("Remote URL:")?;

    if url.trim().is_empty() {
        println!("  Cancelled.");
        return Ok(());
    }

    // Confirm
    println!();
    let confirm_add = confirm(&format!("Run: git remote add origin {}", url))?;
    if !confirm_add {
        println!("  Cancelled.");
        return Ok(());
    }

    GitInfo::remote_add("origin", &url)?;
    println!("  {} Remote origin added", CHECK);

    // Ask to push and set upstream
    let current_branch = info.current_branch.unwrap_or_else(|| "main".to_string());
    println!();
    let push = confirm(&format!(
        "Push current branch and set upstream to origin/{}?",
        current_branch
    ))?;

    if push {
        GitInfo::push_u("origin", &current_branch)?;
        println!("  {} Pushed and set upstream", CHECK);
    }

    Ok(())
}
