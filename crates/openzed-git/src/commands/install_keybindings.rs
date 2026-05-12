use crate::core::config::{suggested_keybindings, zed_keymap_path};
use crate::ui::aura::aura::{CYAN, TEXT, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_text, error_msg, success, warning_msg};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Keybindings");

    let keymap_path = zed_keymap_path();

    println!();
    print!("  ");
    aura_text("Zed keymap path: ", TEXT_MUTED);
    aura_text(keymap_path.to_string_lossy().as_ref(), CYAN);
    println!();

    // Check if keymap.json exists
    if keymap_path.exists() {
        println!();
        aura_text("Keybindings file exists. What would you like to do?", TEXT);
        println!();

        let choices = [
            "Show suggested keybindings",
            "Append safely to keymap",
            "Print snippet only",
            "View current keymap",
            "Cancel",
        ];

        let selection = dialoguer::Select::new()
            .with_prompt("Select an option")
            .items(&choices)
            .default(0)
            .interact()?;

        match selection {
            0 => show_suggested()?,
            1 => append_keybindings(&keymap_path)?,
            2 => print_snippet()?,
            3 => view_current(&keymap_path)?,
            4 => {
                println!();
                print!("  ");
                aura_text("Cancelled.", TEXT_MUTED);
                println!();
            }
            _ => {}
        }
    } else {
        println!();
        warning_msg("Keymap file does not exist.");
        println!();

        let create = dialoguer::Confirm::new()
            .with_prompt("Create keymap.json with suggested keybindings?")
            .default(false)
            .interact()?;

        if create {
            create_keymap(&keymap_path)?;
        } else {
            println!();
            print!("  ");
            aura_text(
                "Cancelled. You can print the snippet and add manually.",
                TEXT_MUTED,
            );
            println!();
            println!();
            print_snippet()?;
        }
    }

    Ok(())
}

fn show_suggested() -> Result<()> {
    println!();
    aura_text("Suggested Keybindings:", CYAN);
    println!();
    println!("  cmd-alt-g m - OpenZed Git: Menu");
    println!("  cmd-alt-g s - OpenZed Git: Status+");
    println!("  cmd-alt-g g - OpenZed Git: Git Graph");
    println!("  cmd-alt-g c - OpenZed Git: Commit Assistant");
    println!("  cmd-alt-g p - OpenZed Git: Publish to GitHub");
    println!("  cmd-alt-g r - OpenZed Git: Pull Requests");
    println!();
    aura_text("These are Workspace-level bindings.", TEXT_MUTED);
    Ok(())
}

fn print_snippet() -> Result<()> {
    println!();
    aura_text("Copy this snippet to your keymap.json:", CYAN);
    println!();
    println!("{}", suggested_keybindings());
    println!();
    aura_text("Add it as a new element in the array.", TEXT_MUTED);
    Ok(())
}

fn append_keybindings(keymap_path: &std::path::Path) -> Result<()> {
    println!();
    print!("  Reading existing keymap... ");

    let content = std::fs::read_to_string(keymap_path)?;

    // Parse JSON to check structure
    let mut json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            println!();
            error_msg(&format!("Invalid JSON: {}", e));
            println!();
            print!("  ");
            aura_text(
                "Use 'Print snippet only' to get the raw snippet.",
                TEXT_MUTED,
            );
            println!();
            return Ok(());
        }
    };

    // Parse suggested bindings
    let suggested: serde_json::Value = match serde_json::from_str(&suggested_keybindings()) {
        Ok(v) => v,
        Err(e) => {
            error_msg(&format!("Failed to parse suggested keybindings: {}", e));
            return Ok(());
        }
    };

    // Get the bindings from suggested
    let suggested_bindings = &suggested[0]["bindings"];

    // Find Workspace context or add it
    let is_array = json.is_array();

    if is_array {
        // Check if there's a Workspace context entry
        let mut found = false;
        if let Some(arr) = json.as_array_mut() {
            for item in arr.iter_mut() {
                if let Some(ctx) = item.get("context").and_then(|c| c.as_str()) {
                    if ctx == "Workspace" {
                        // Merge bindings
                        if let Some(existing) =
                            item.get_mut("bindings").and_then(|b| b.as_object_mut())
                        {
                            if let Some(new_bindings) = suggested_bindings.as_object() {
                                for (key, val) in new_bindings {
                                    existing.insert(key.clone(), val.clone());
                                }
                            }
                        }
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                // Add new Workspace context entry
                arr.push(serde_json::json!({
                    "context": "Workspace",
                    "bindings": suggested_bindings
                }));
            }
        }

        let new_content = serde_json::to_string_pretty(&json)
            .map_err(|e| anyhow::anyhow!("Failed to serialize: {}", e))?;

        std::fs::write(keymap_path, new_content)?;

        success("Keybindings appended successfully.");
        println!();
        print!("  ");
        aura_text("Restart Zed to use new keybindings.", TEXT_MUTED);
        println!();
    } else {
        // Not an array, likely an object with contexts
        warning_msg("Keymap structure not recognized as array.");
        print!("  ");
        aura_text("Use 'Print snippet only' and add manually.", TEXT_MUTED);
        println!();
    }

    Ok(())
}

fn view_current(keymap_path: &std::path::Path) -> Result<()> {
    println!();
    print!("  Reading keymap... ");

    match std::fs::read_to_string(keymap_path) {
        Ok(content) => {
            println!();
            aura_text("Current keymap.json contents:", CYAN);
            println!();
            // Pretty print
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&content) {
                let formatted =
                    serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| content.clone());
                println!("{}", formatted);
            } else {
                println!("{}", content);
            }
        }
        Err(e) => {
            println!();
            error_msg(&format!("Failed to read keymap: {}", e));
        }
    }

    Ok(())
}

fn create_keymap(keymap_path: &std::path::Path) -> Result<()> {
    // Create directory if needed
    if let Some(parent) = keymap_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create initial keymap with suggested bindings
    let initial: serde_json::Value = serde_json::from_str(&suggested_keybindings())
        .map_err(|e| anyhow::anyhow!("Failed to parse suggested: {}", e))?;

    let content = serde_json::to_string_pretty(&initial)
        .map_err(|e| anyhow::anyhow!("Failed to format: {}", e))?;

    std::fs::write(keymap_path, content)?;

    success("Keymap created successfully.");
    println!();
    print!("  ");
    aura_text("Restart Zed to use keybindings.", TEXT_MUTED);
    println!();

    Ok(())
}
