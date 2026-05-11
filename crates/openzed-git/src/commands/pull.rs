use crate::core::git::GitInfo;
use crate::ui::{colors::*, output::separator};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Pull");

    let info = GitInfo::get()?;

    if info.remote_origin.is_none() {
        println!("  {} No remote origin found.", WARNING);
        println!("  Run: openzed-git publish");
        return Ok(());
    }

    print!("  Pulling... ");
    if GitInfo::pull().is_ok() {
        println!("{}", CHECK);
    } else {
        println!("{}", WARNING);
        println!("  Pull completed with conflicts.");
        println!("  Resolve conflicts in Zed, then run:");
        println!("  openzed-git commit-push");
    }

    Ok(())
}
