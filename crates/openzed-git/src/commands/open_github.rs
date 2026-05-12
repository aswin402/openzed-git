use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::ui::aura::separator;
use crate::ui::aura::aura::WARNING;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Open GitHub");

    let info = GitInfo::get()?;

    // Check remote
    if info.remote_origin.is_none() {
        println!("  {} No remote origin found.", WARNING);
        println!("  Run: openzed-git publish");
        return Ok(());
    }

    let remote = info.remote_origin.unwrap();
    let url = GithubInfo::remote_to_github_url(&remote);

    match url {
        Some(github_url) => {
            println!("  Opening: {}", github_url);
            open::that(&github_url)?;
        }
        None => {
            println!("  {} Could not parse remote URL.", WARNING);
            println!("  Remote: {}", remote);
        }
    }

    Ok(())
}
