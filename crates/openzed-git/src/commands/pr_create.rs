use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::{CHECK, CYAN, GREEN, PURPLE, TEXT_MUTED, YELLOW};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use crate::ui::prompts::input;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Create Pull Request");

    // Check if it's a git repo
    if GitInfo::is_repo().is_err() {
        error_msg("Not a Git repository.");
        return Ok(());
    }

    // Check GitHub CLI
    if let Err(e) = GithubInfo::ensure_gh_ready() {
        error_msg("GitHub CLI not ready.");
        aura_text(&format!(" {}", e), TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Check remote origin
    let remote_origin = match GitInfo::remote_origin() {
        Ok(r) => r,
        Err(_) => {
            error_msg("No origin remote found.");
            print!("  ");
            aura_text("Run: openzed-git setup-remote", TEXT_MUTED);
            println!();
            return Ok(());
        }
    };

    // Check if origin is GitHub
    if GithubInfo::remote_to_github_url(&remote_origin).is_none() {
        error_msg("Origin is not a GitHub repository.");
        return Ok(());
    }

    // Get current branch
    let current_branch = match GitInfo::current_branch() {
        Ok(b) => b,
        Err(_) => {
            error_msg("Not on a branch.");
            return Ok(());
        }
    };

    println!();
    aura_text("Current branch: ", TEXT_MUTED);
    aura_text(&current_branch, CYAN);
    println!();

    // Check if current branch has upstream
    let has_upstream = GitInfo::upstream()?.is_some();
    if !has_upstream {
        warning_msg("Current branch has no upstream.");
        println!();

        let push_now = confirm("Push current branch and set upstream?")?;
        if !push_now {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }

        print!("  Pushing and setting upstream... ");
        if let Err(e) = GithubInfo::push_current_branch() {
            println!();
            error_msg(&format!("Failed to push: {}", e));
            return Ok(());
        }
        aura_text(CHECK.trim(), GREEN);
        println!();
    }

    // Get base branch (try to detect default from remote)
    let base = "main";

    println!();
    let base_branch: String = input(&format!("Base branch [{}]:", base))
        .unwrap_or_else(|_| base.to_string())
        .trim()
        .to_string();
    let base = if base_branch.is_empty() {
        "main"
    } else {
        &base_branch
    };

    // Check if current branch is same as base
    if current_branch == base {
        warning_msg("You are currently on the base branch.");
        println!();
        aura_text("Create a feature branch before opening a PR.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Get PR title
    println!();
    let title: String = match input("PR title:") {
        Ok(t) => t.trim().to_string(),
        Err(_) => {
            println!();
            print!("  ");
            aura_text("Cancelled.", TEXT_MUTED);
            println!();
            return Ok(());
        }
    };

    if title.is_empty() {
        error_msg("PR title cannot be empty.");
        return Ok(());
    }

    // Get PR body
    println!();
    let body: String = input("PR body (optional, press Enter to skip):")
        .unwrap_or_default()
        .trim()
        .to_string();

    // Draft PR
    println!();
    let is_draft = confirm("Create as draft PR?")?;

    // Preview
    println!();
    separator("OpenZed Git: PR Preview");
    println!();

    aura_text("Base: ", TEXT_MUTED);
    aura_text(base, CYAN);
    println!();

    aura_text("Head: ", TEXT_MUTED);
    aura_text(&current_branch, CYAN);
    println!();

    aura_text("Title: ", TEXT_MUTED);
    aura_text(&title, PURPLE);
    println!();

    if !body.is_empty() {
        aura_text("Body: ", TEXT_MUTED);
        aura_text(&body, TEXT_MUTED);
        println!();
    }

    aura_text("Draft: ", TEXT_MUTED);
    if is_draft {
        aura_text("yes", YELLOW);
    } else {
        aura_text("no", GREEN);
    }
    println!();

    // Confirm
    println!();
    let create = confirm("Create pull request?")?;
    if !create {
        println!();
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    // Create PR
    print!("  Creating PR... ");

    let body_opt = if body.is_empty() {
        None
    } else {
        Some(body.as_str())
    };

    match GithubInfo::pr_create(base, &current_branch, &title, body_opt, is_draft) {
        Ok(url) => {
            aura_text(CHECK.trim(), GREEN);
            println!();
            success("Pull request created successfully!");
            println!();

            aura_text("PR URL: ", TEXT_MUTED);
            aura_text(&url, PURPLE);
            println!();

            // Open in browser
            println!();
            let open_browser = confirm("Open in browser?")?;
            if open_browser {
                print!("  Opening in browser... ");
                if let Err(e) = GithubInfo::pr_view_web_current() {
                    println!();
                    error_msg(&format!("Failed to open: {}", e));
                } else {
                    aura_text(CHECK.trim(), GREEN);
                    println!();
                }
            }
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to create PR: {}", e));
        }
    }

    Ok(())
}
