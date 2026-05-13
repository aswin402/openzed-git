use crate::core::git::GitInfo;
use crate::ui::aura::aura::CHECK;
use crate::ui::aura::separator;
use anyhow::Result;

fn is_interactive() -> bool {
    atty::is(atty::Stream::Stdin)
}

fn ask_yes_no(prompt: &str) -> bool {
    if is_interactive() {
        crate::core::shell::confirm(prompt).unwrap_or(false)
    } else {
        false
    }
}

fn ask_commit_message() -> String {
    if is_interactive() {
        crate::ui::prompts::input_with_default("Commit message:", "Update changes")
            .unwrap_or_else(|_| "Update changes".to_string())
    } else {
        "Update changes".to_string()
    }
}

fn ask_commit_body() -> Option<String> {
    if is_interactive() {
        let body = crate::ui::prompts::input_with_default(
            "Commit body (optional, press Enter to skip):",
            "",
        )
        .unwrap_or_default();
        if body.is_empty() {
            None
        } else {
            Some(body)
        }
    } else {
        None
    }
}

pub fn run() -> Result<()> {
    separator("OpenZed Git: Commit + Push");

    let info = GitInfo::get()?;

    // Check if there are staged files
    let status = GitInfo::status()?;
    let has_staged = status.lines().skip(1).any(|line| {
        !line.starts_with("??") && line.chars().nth(1).map_or(false, |c| c != ' ')
    });

    if !has_staged {
        println!();
        println!("  No staged files found.");
        if is_interactive() {
            let stage_all = ask_yes_no("Stage all changes?");
            if stage_all {
                GitInfo::add_all()?;
                println!("  {} All changes staged", CHECK);
            } else {
                println!("  Cancelled.");
                return Ok(());
            }
        } else {
            println!("  Auto-staging all changes (non-interactive mode)...");
            GitInfo::add_all()?;
            println!("  {} All changes staged", CHECK);
        }
    }

    // Get commit message
    println!();
    let message = ask_commit_message();

    // Optional body (interactive only)
    let body = ask_commit_body();

    // Commit
    print!("  Committing... ");
    if body.is_some() {
        GitInfo::create_commit(&message, body.as_deref())?;
    } else {
        GitInfo::commit(&message)?;
    }
    println!("{}", CHECK);

    // Detect upstream
    let upstream = GitInfo::upstream()?;

    if upstream.is_some() {
        println!();
        if is_interactive() {
            let push = ask_yes_no("Push commit now?");
            if push {
                print!("  Pushing... ");
                GitInfo::push()?;
                println!("{}", CHECK);
            }
        } else {
            print!("  Pushing to {}... ", upstream.as_ref().unwrap());
            GitInfo::push()?;
            println!("{}", CHECK);
        }
    } else {
        let current_branch = info.current_branch.unwrap_or_else(|| "main".to_string());
        println!();
        println!("  No upstream branch found.");
        if is_interactive() {
            let set_upstream = ask_yes_no(&format!(
                "Push and set upstream to origin/{}?",
                current_branch
            ));
            if set_upstream {
                print!("  Pushing with upstream... ");
                GitInfo::push_u("origin", &current_branch)?;
                println!("{}", CHECK);
            }
        } else {
            print!("  Pushing with upstream to origin/{}... ", current_branch);
            GitInfo::push_u("origin", &current_branch)?;
            println!("{}", CHECK);
        }
    }

    Ok(())
}