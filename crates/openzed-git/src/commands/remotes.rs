use crate::core::git::GitInfo;
use crate::ui::aura::aura::WARNING;
use crate::ui::aura::separator;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Remotes");

    let remotes = GitInfo::remotes()?;

    if remotes.trim().is_empty() {
        println!("  {} No remotes found.", WARNING);
        println!();
        println!("  Run: openzed-git publish");
        println!("  or:");
        println!("  openzed-git setup-remote");
    } else {
        println!("{}", remotes);
    }

    Ok(())
}
