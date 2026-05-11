use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::{colors::*, output::separator, prompts::input_with_default};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Set Upstream");

    let info = GitInfo::get()?;

    // Check remote
    if info.remote_origin.is_none() {
        println!("  {} No remote origin found.", WARNING);
        println!("  Run: openzed-git publish");
        return Ok(());
    }

    let current_branch = info.current_branch.unwrap_or_else(|| "main".to_string());
    let upstream_branch: String =
        input_with_default("Remote branch name:", &current_branch)?;

    // Confirm
    println!();
    let confirm_set = confirm(&format!("Run: git push -u origin {}", upstream_branch))?;

    if !confirm_set {
        println!("  Cancelled.");
        return Ok(());
    }

    GitInfo::push_u("origin", &upstream_branch)?;
    println!("  {} Upstream set to origin/{}", CHECK, upstream_branch);

    Ok(())
}
