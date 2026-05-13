use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{CHECK, CROSS, CYAN, GREEN, RED, TEXT_DIM, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, link, success, warning_msg};
use crate::ui::prompts::{input_with_default, select_with_default};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Publish to GitHub");

    let cwd = std::env::current_dir()?;

    // Check if git repo, auto-init if not
    print!("  Checking Git... ");
    if GitInfo::is_repo().is_ok() {
        print!(" ");
        aura_text(CHECK.trim(), GREEN);
        println!();
    } else {
        print!(" ");
        aura_text(CROSS.trim(), RED);
        println!();
        print!("  Initializing Git repository... ");
        GitInfo::init()?;
        print!(" ");
        aura_text(CHECK.trim(), GREEN);
        println!();
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
            success("Initial commit created");
        }
    }

    // Check gh
    println!();
    print!("  Checking GitHub CLI... ");
    if GithubInfo::is_installed().unwrap_or(false) {
        print!(" ");
        aura_text(CHECK.trim(), GREEN);
        println!();
    } else {
        print!(" ");
        aura_text(CROSS.trim(), RED);
        println!();
        error_msg("GitHub CLI is not installed.");
        print!("    ");
        aura_text("Install it from:", TEXT_MUTED);
        print!(" ");
        aura_text("https://cli.github.com/", CYAN);
        println!();
        return Ok(());
    }

    print!("  Checking GitHub authentication... ");
    if GithubInfo::is_authenticated().unwrap_or(false) {
        print!(" ");
        aura_text(CHECK.trim(), GREEN);
        println!();
    } else {
        print!(" ");
        aura_text(CROSS.trim(), RED);
        println!();
        error_msg("GitHub CLI is not authenticated.");
        print!("    ");
        aura_text("Run:", TEXT_MUTED);
        print!(" ");
        aura_text("gh auth login", CYAN);
        println!();
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

    let visibility = select_with_default("Visibility", &["public", "private"], 0)?;

    let public = visibility == 0;

    // Check if remote exists
    if info.remote_origin.is_some() {
        println!();
        warning_msg("Remote origin already exists");
        print!("    ");
        aura_text(&info.remote_origin.as_ref().unwrap(), TEXT_DIM);
        println!();
        let choice = select_with_default(
            "What do you want to do?",
            &["Use existing remote and push", "Cancel"],
            1,
        )?;

        if choice == 1 {
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }
    } else {
        // Create repo
        println!();
        print!("  Creating repository on GitHub... ");
        GithubInfo::create_repo(&name, Some(&description), public, &cwd)?;
        print!(" ");
        aura_text(CHECK.trim(), GREEN);
        println!();
    }

    // Push
    println!();
    print!("  Setting remote origin... ");
    print!(" ");
    aura_text(CHECK.trim(), GREEN);
    println!();
    print!("  Pushing branch... ");
    print!(" ");
    aura_text(CHECK.trim(), GREEN);
    println!();

    // Show result
    println!();
    success("Repository published successfully!");

    // Build GitHub URL
    let remote = GitInfo::remote_origin().ok();
    let github_url = remote.and_then(|r| GithubInfo::remote_to_github_url(&r));

    if let Some(url) = github_url {
        link(&url);

        let open = confirm("Open in browser?")?;
        if open {
            open::that(&url)?;
        }
    }

    Ok(())
}
