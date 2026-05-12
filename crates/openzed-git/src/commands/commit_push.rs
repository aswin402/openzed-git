use crate::core::git::GitInfo;
use crate::core::shell::confirm;
use crate::ui::aura::aura::CHECK;
use crate::ui::aura::separator;
use crate::ui::prompts::input_with_default;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Commit + Push");

    let info = GitInfo::get()?;

    // Check if there are staged files
    let status = GitInfo::status()?;
    let has_staged = status.lines().any(|line| {
        line.len() >= 2
            && line.chars().next().unwrap_or(' ') != ' '
            && line.chars().next().unwrap_or(' ') != '?'
    });

    if !has_staged {
        println!();
        println!("  No staged files found.");
        let stage_all = confirm("Stage all changes?")?;
        if stage_all {
            GitInfo::add_all()?;
            println!("  {} All changes staged", CHECK);
        } else {
            println!("  Cancelled.");
            return Ok(());
        }
    }

    // Get commit message
    println!();
    let message: String = input_with_default("Commit message:", "Update changes")?;

    // Optional body
    let body: String = input_with_default("Commit body (optional, press Enter to skip):", "")?;

    // Commit
    print!("  Committing... ");
    if body.is_empty() {
        GitInfo::commit(&message)?;
    } else {
        // Use git commit -m message -m body
        std::process::Command::new("git")
            .args(["commit", "-m", &message, "-m", &body])
            .output()?;
    }
    println!("{}", CHECK);

    // Detect upstream
    let upstream = GitInfo::upstream()?;

    if upstream.is_some() {
        println!();
        let push = confirm("Push commit now?")?;
        if push {
            print!("  Pushing... ");
            GitInfo::push()?;
            println!("{}", CHECK);
        }
    } else {
        let current_branch = info.current_branch.unwrap_or_else(|| "main".to_string());
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
        }
    }

    Ok(())
}
