use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::ui::{colors::*, output::separator};
use anyhow::Result;
use std::process::Command;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Doctor");

    let mut all_pass = true;
    let mut git_installed = false;
    let mut gh_installed = false;
    let mut gh_authenticated = false;
    let mut is_repo = false;
    let mut has_user_name = false;
    let mut has_user_email = false;
    let mut has_remote = false;
    let mut has_upstream = false;

    // Check git
    print!("  Git installed... ");
    if Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        println!("{}", CHECK);
        git_installed = true;
    } else {
        println!("{}", CROSS);
        all_pass = false;
    }

    // Check GitHub CLI
    print!("  GitHub CLI installed... ");
    if GithubInfo::is_installed().unwrap_or(false) {
        println!("{}", CHECK);
        gh_installed = true;
    } else {
        println!("{}", WARNING);
    }

    // Check GitHub auth
    print!("  GitHub authenticated... ");
    if GithubInfo::is_authenticated().unwrap_or(false) {
        println!("{}", CHECK);
        gh_authenticated = true;
    } else {
        println!("{}", WARNING);
    }

    // Check repo
    print!("  Git repository... ");
    if GitInfo::is_repo().unwrap_or(false) {
        println!("{}", CHECK);
        is_repo = true;
    } else {
        println!("{}", CROSS);
        all_pass = false;
    }

    if is_repo {
        // Check user.name
        print!("  user.name configured... ");
        if GitInfo::user_name().is_ok() {
            println!("{}", CHECK);
            has_user_name = true;
        } else {
            println!("{}", CROSS);
            all_pass = false;
        }

        // Check user.email
        print!("  user.email configured... ");
        if GitInfo::user_email().is_ok() {
            println!("{}", CHECK);
            has_user_email = true;
        } else {
            println!("{}", CROSS);
            all_pass = false;
        }

        // Check remote
        print!("  Remote origin exists... ");
        if GitInfo::remote_origin().is_ok() {
            println!("{}", CHECK);
            has_remote = true;
        } else {
            println!("{}", WARNING);
        }

        // Check upstream
        print!("  Upstream branch set... ");
        if GitInfo::upstream().unwrap_or(None).is_some() {
            println!("{}", CHECK);
            has_upstream = true;
        } else {
            println!("{}", WARNING);
        }
    }

    println!();
    if all_pass {
        println!("  {} All checks passed!", CHECK);
    } else {
        println!("  {} Some checks failed. Fix the issues above.", WARNING);
    }

    Ok(())
}
