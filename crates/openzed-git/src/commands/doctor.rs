use crate::core::git::GitInfo;
use crate::core::github::GithubInfo;
use crate::ui::aura::aura::{CHECK, CROSS, WARNING};
use crate::ui::aura::separator;
use anyhow::Result;
use std::process::Command;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Doctor");

    let mut all_pass = true;

    // Check git
    print!("  Git installed... ");
    if Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        println!("{}", CHECK);
    } else {
        println!("{}", CROSS);
        all_pass = false;
    }

    // Check GitHub CLI
    print!("  GitHub CLI installed... ");
    if GithubInfo::is_installed().unwrap_or(false) {
        println!("{}", CHECK);
    } else {
        println!("{}", WARNING);
    }

    // Check GitHub auth
    print!("  GitHub authenticated... ");
    if GithubInfo::is_authenticated().unwrap_or(false) {
        println!("{}", CHECK);
    } else {
        println!("{}", WARNING);
    }

    // Check repo
    print!("  Git repository... ");
    if GitInfo::is_repo().unwrap_or(false) {
        println!("{}", CHECK);
    } else {
        println!("{}", CROSS);
        all_pass = false;
    }

    if GitInfo::is_repo().unwrap_or(false) {
        // Check user.name
        print!("  user.name configured... ");
        if GitInfo::user_name().is_ok() {
            println!("{}", CHECK);
        } else {
            println!("{}", CROSS);
            all_pass = false;
        }

        // Check user.email
        print!("  user.email configured... ");
        if GitInfo::user_email().is_ok() {
            println!("{}", CHECK);
        } else {
            println!("{}", CROSS);
            all_pass = false;
        }

        // Check remote
        print!("  Remote origin exists... ");
        if GitInfo::remote_origin().is_ok() {
            println!("{}", CHECK);
        } else {
            println!("{}", WARNING);
        }

        // Check upstream
        print!("  Upstream branch set... ");
        if GitInfo::upstream().unwrap_or(None).is_some() {
            println!("{}", CHECK);
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
