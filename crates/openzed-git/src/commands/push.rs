use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::separator;
use crate::ui::aura::aura::{CHECK, WARNING};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Push / Set Upstream");

    let info = GitInfo::get()?;

    // Check remote
    if info.remote_origin.is_none() {
        println!();
        println!("  {} No remote origin found.", WARNING);
        println!();
        println!("  Run: openzed-git publish");
        return Ok(());
    }

    // Detect upstream
    let upstream = GitInfo::upstream()?;
    let current_branch = info.current_branch.unwrap_or_else(|| "main".to_string());

    if upstream.is_some() {
        print!("  Pushing to {}... ", upstream.as_ref().unwrap());
        GitInfo::push()?;
        println!("{}", CHECK);
    } else {
        println!();
        println!("  No upstream branch found.");
        let set_upstream = confirm(&format!(
            "Push and set upstream to origin/{}?",
            current_branch
        ))?;
        if set_upstream {
            print!("  Pushing with upstream... ");
            GitInfo::push_u("origin", &current_branch)?;
            println!("{}", CHECK);
        } else {
            println!("  Cancelled.");
        }
    }

    Ok(())
}
