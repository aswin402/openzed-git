use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::core::shell::confirm;
use crate::ui::{colors::*, output::separator, prompts::input_with_default};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Publish to GitHub");

    let cwd = std::env::current_dir()?;

    // Check if git repo, auto-init if not
    print!("  Checking git... ");
    if GitInfo::is_repo().is_ok() {
        println!("{}", CHECK);
    } else {
        println!("{}", CROSS);
        print!("  Initializing Git repository... ");
        GitInfo::init()?;
        println!("{}", CHECK);
    }

    let info = GitInfo::get()?;

    // Ask for default branch
    println!();
    let branch: String = input_with_default("Default branch:", "main")?;

    // Only prompt to rename if there's an existing branch (i.e., commits exist)
    let has_existing_branch = info.current_branch.is_some();
    if has_existing_branch && info.current_branch.as_ref() != Some(&branch) {
        let rename = confirm(&format!("Rename current branch to '{}'?", branch))?;
        if rename {
            if let Some(ref current) = info.current_branch {
                GitInfo::branch_rename(current, &branch)?;
            }
        }
    }

    // Check if commits exist
    let has_commits = GitInfo::has_commits().unwrap_or(false);
    if !has_commits {
        println!();
        let create_commit =
            confirm("No commits found. Stage all files and create initial commit?")?;
        if create_commit {
            let message: String = input_with_default("Initial commit message:", "initial commit")?;
            GitInfo::add_all()?;
            GitInfo::commit(&message)?;
            println!("  {} Initial commit created", CHECK);
        }
    }

    // Check gh
    println!();
    print!("  Checking GitHub CLI... ");
    if GithubInfo::is_installed().unwrap_or(false) {
        println!("{}", CHECK);
    } else {
        println!("{}", CROSS);
        println!();
        println!("  GitHub CLI is not installed.");
        println!("  Install it from: https://cli.github.com/");
        println!("  Then run: gh auth login");
        return Ok(());
    }

    print!("  Checking GitHub authentication... ");
    if GithubInfo::is_authenticated().unwrap_or(false) {
        println!("{}", CHECK);
    } else {
        println!("{}", CROSS);
        println!();
        println!("  GitHub CLI is not authenticated.");
        println!("  Run: gh auth login");
        return Ok(());
    }

    // Ask repo details
    println!();
    let default_name = cwd
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "my-repo".to_string());

    let name: String = input_with_default("Repository name:", &default_name)?;
    let description: String = input_with_default("Description (optional):", "")?;

    let visibility = dialoguer::Select::new()
        .with_prompt("Visibility")
        .items(&["public", "private"])
        .default(0)
        .interact()?;

    let public = visibility == 0;

    // Check if remote exists
    if info.remote_origin.is_some() {
        println!();
        println!("  Remote origin already exists:");
        println!("    {}", info.remote_origin.as_ref().unwrap());
        let choice = dialoguer::Select::new()
            .with_prompt("What do you want to do?")
            .items(&["Use existing remote and push", "Cancel"])
            .default(1)
            .interact()?;

        if choice == 1 {
            println!("  Cancelled.");
            return Ok(());
        }
    } else {
        // Create repo
        println!();
        print!("  Creating repository on GitHub... ");
        GithubInfo::create_repo(&name, Some(&description), public, &cwd)?;
        println!("{}", CHECK);
    }

    // Push - already done by gh create --push
    println!();
    println!("{} Pushed to GitHub", CHECK);

    // Show result
    println!();
    println!("  {} Repository published successfully!", CHECK);

    // Build GitHub URL
    let remote = GitInfo::remote_origin().ok();
    let github_url = remote.and_then(|r| GithubInfo::remote_to_github_url(&r));

    if let Some(url) = github_url {
        println!();
        println!("  GitHub: {}", url);

        let open = confirm("Open in browser?")?;
        if open {
            open::that(&url)?;
        }
    }

    Ok(())
}
