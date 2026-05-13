use crate::core::config::{
    create_global_config, create_project_config, global_config_path, load_config,
    project_config_path,
};
use crate::core::shell::confirm;
use crate::ui::aura::aura::{CYAN, TEXT, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, labeled_value, success, warning_msg};
use crate::ui::prompts::select_with_default;
use anyhow::Result;
use std::process::Command;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Config");

    loop {
        println!();
        aura_text("What do you want to do?", TEXT);
        println!();

        let selection = select_with_default(
            "Select an option",
            &[
                "Show current config",
                "Create global config",
                "Create project config",
                "Edit global config",
                "Edit project config",
                "Reset global config",
                "Cancel",
            ],
            0,
        )?;

        match selection {
            0 => show_config()?,
            1 => create_global()?,
            2 => create_project()?,
            3 => edit_global()?,
            4 => edit_project()?,
            5 => reset_global()?,
            6 => {
                println!();
                print!("  ");
                aura_text("Cancelled.", TEXT_MUTED);
                println!();
                break;
            }
            _ => {}
        }

        // Ask if user wants to continue
        println!();
        if !confirm("Return to config menu?")? {
            break;
        }
    }

    Ok(())
}

fn show_config() -> Result<()> {
    println!();
    aura_text("Current Configuration", CYAN);
    println!();

    let config = load_config()?;

    labeled_value("Default branch", &config.default_branch);
    labeled_value("Default visibility", &config.default_visibility);
    labeled_value("Auto open browser", &config.auto_open_browser.to_string());
    labeled_value("Theme", &config.theme);
    labeled_value("Commit style", &config.commit_style);
    labeled_value("Default PR base", &config.default_pr_base);
    labeled_value("Remote", &config.remote);

    if let Some(owner) = config.github_owner {
        labeled_value("GitHub owner", &owner);
    }

    println!();
    aura_text("Config file locations:", TEXT_MUTED);
    println!();
    print!("  Global:  ");
    aura_text(global_config_path().to_string_lossy().as_ref(), CYAN);
    println!();
    print!("  Project: ");
    aura_text(project_config_path().to_string_lossy().as_ref(), CYAN);
    println!();

    println!();
    aura_text("(Project config overrides global config)", TEXT_MUTED);

    Ok(())
}

fn create_global() -> Result<()> {
    let path = global_config_path();

    if path.exists() {
        warning_msg("Global config already exists.");
        print!("  ");
        aura_text("Use 'Edit global config' to modify it.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Creating global config at: ");
    aura_text(path.to_string_lossy().as_ref(), CYAN);
    println!();

    match create_global_config() {
        Ok(_) => {
            success("Global config created successfully.");
            println!();
            print!("  ");
            aura_text(
                "Edit it with 'openzed-git config' -> Edit global config",
                TEXT_MUTED,
            );
            println!();
        }
        Err(e) => {
            error_msg(&format!("Failed to create global config: {}", e));
        }
    }

    Ok(())
}

fn create_project() -> Result<()> {
    let path = project_config_path();

    if path.exists() {
        warning_msg("Project config already exists.");
        print!("  ");
        aura_text("Use 'Edit project config' to modify it.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Creating project config at: ");
    aura_text(path.to_string_lossy().as_ref(), CYAN);
    println!();

    match create_project_config() {
        Ok(_) => {
            success("Project config created successfully.");
            println!();
            print!("  ");
            aura_text(
                "Edit it with 'openzed-git config' -> Edit project config",
                TEXT_MUTED,
            );
            println!();
        }
        Err(e) => {
            error_msg(&format!("Failed to create project config: {}", e));
        }
    }

    Ok(())
}

fn edit_global() -> Result<()> {
    let path = global_config_path();

    if !path.exists() {
        warning_msg("Global config does not exist.");
        print!("  ");
        aura_text("Use 'Create global config' first.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Opening: ");
    aura_text(path.to_string_lossy().as_ref(), CYAN);
    println!();

    // Try to open with zed, then vim as fallback
    let result = Command::new("zed").arg(&path).output();

    if result.is_err() || !result.as_ref().unwrap().status.success() {
        // Fallback to vim
        let result = Command::new("vim").arg(&path).output();
        if result.is_err() {
            error_msg("Failed to open editor.");
            println!();
            print!("  ");
            aura_text("Open manually: ", TEXT_MUTED);
            aura_text(path.to_string_lossy().as_ref(), CYAN);
            println!();
        }
    }

    Ok(())
}

fn edit_project() -> Result<()> {
    let path = project_config_path();

    if !path.exists() {
        warning_msg("Project config does not exist.");
        print!("  ");
        aura_text("Use 'Create project config' first.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    print!("  Opening: ");
    aura_text(path.to_string_lossy().as_ref(), CYAN);
    println!();

    // Try to open with zed, then vim as fallback
    let result = Command::new("zed").arg(&path).output();

    if result.is_err() || !result.as_ref().unwrap().status.success() {
        // Fallback to vim
        let result = Command::new("vim").arg(&path).output();
        if result.is_err() {
            error_msg("Failed to open editor.");
            println!();
            print!("  ");
            aura_text("Open manually: ", TEXT_MUTED);
            aura_text(path.to_string_lossy().as_ref(), CYAN);
            println!();
        }
    }

    Ok(())
}

fn reset_global() -> Result<()> {
    let path = global_config_path();

    if !path.exists() {
        warning_msg("Global config does not exist. Nothing to reset.");
        return Ok(());
    }

    println!();
    print!("  ");
    aura_text(
        "WARNING: This will delete your global config at:",
        TEXT_MUTED,
    );
    println!();
    print!("    ");
    aura_text(path.to_string_lossy().as_ref(), CYAN);
    println!();

    let confirm_reset = confirm("Are you sure you want to reset global config?")?;
    if !confirm_reset {
        print!("  ");
        aura_text("Cancelled.", TEXT_MUTED);
        println!();
        return Ok(());
    }

    std::fs::remove_file(&path).map_err(|e| anyhow::anyhow!("{}", e))?;
    success("Global config has been reset.");

    Ok(())
}
