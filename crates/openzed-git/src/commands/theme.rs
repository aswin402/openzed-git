use crate::core::config::{global_config_path, load_config};
use crate::ui::aura::aura::{ARROW, CYAN, PINK, PURPLE, TEXT, TEXT_MUTED};
use crate::ui::aura::separator;
use crate::ui::aura::{aura_bold, aura_print, aura_text, error_msg, link, success, warning_msg};
use crate::ui::prompts::select_with_default;
use anyhow::Result;

/// Theme definition
struct Theme {
    name: &'static str,
    description: &'static str,
}

const THEMES: &[Theme] = &[
    Theme {
        name: "aura-dark",
        description: "Aura Dark - Purple/pink accent, dark background",
    },
    Theme {
        name: "minimal-dark",
        description: "Minimal Dark - Clean, simple dark theme",
    },
    Theme {
        name: "zed-dark",
        description: "Zed Dark - Match Zed editor's dark theme",
    },
];

pub fn run() -> Result<()> {
    separator("OpenZed Git: Theme");

    // Show current theme
    let config = load_config()?;
    print!("  ");
    aura_text("Current theme: ", TEXT_MUTED);
    aura_text(&config.theme, CYAN);
    println!();

    loop {
        println!();
        aura_text("What do you want to do?", TEXT);
        println!();

        let selection = select_with_default(
            "Select an option",
            &[
                "List themes",
                "Preview theme",
                "Set theme",
                "Cancel",
            ],
            0,
        )?;

        match selection {
            0 => list_themes()?,
            1 => preview_theme()?,
            2 => set_theme()?,
            3 => {
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
        use crate::core::shell::confirm;

        if !confirm("Return to theme menu?")? {
            break;
        }
    }

    Ok(())
}

fn list_themes() -> Result<()> {
    println!();
    aura_text("Available themes:", CYAN);
    println!();

    for theme in THEMES {
        print!("  ");
        aura_print(ARROW.trim(), PINK);
        aura_text(theme.name, PURPLE);
        print!(" - ");
        aura_text(theme.description, TEXT_MUTED);
        println!();
    }

    Ok(())
}

fn preview_theme() -> Result<()> {
    println!();
    aura_text("Select a theme to preview:", CYAN);
    println!();

    let theme_names: Vec<&str> = THEMES.iter().map(|t| t.name).collect();

    let selection = select_with_default(
        "Which theme?",
        &theme_names,
        0,
    )?;

    let selected = THEMES[selection].name;

    println!();
    separator(&format!("Theme Preview: {}", selected));

    // Show sample UI elements
    println!();
    print!("  ");
    aura_text("Status messages:", TEXT_MUTED);
    println!();
    success("This is a success message");
    warning_msg("This is a warning message");
    error_msg("This is an error message");
    link("https://github.com/example/repo");

    println!();
    print!("  ");
    aura_text("Menu item (selected):", TEXT_MUTED);
    println!();
    print!("  ");
    aura_print(ARROW.trim(), PINK);
    aura_text("Selected item (cyan)", CYAN);
    println!();

    println!();
    print!("  ");
    aura_text("File names and refs:", TEXT_MUTED);
    println!();
    print!("  ");
    aura_text("src/main.rs", CYAN);
    println!();
    print!("  ");
    aura_text("origin/main", PURPLE);
    println!();

    println!();
    print!("  ");
    aura_text("Header title (pink bold):", TEXT_MUTED);
    println!();
    print!("  ");
    aura_bold("Important!", PINK);
    println!();

    Ok(())
}

fn set_theme() -> Result<()> {
    println!();
    aura_text("Select a theme to set:", CYAN);
    println!();

    let theme_names: Vec<&str> = THEMES.iter().map(|t| t.name).collect();

    let selection = select_with_default(
        "Which theme?",
        &theme_names,
        0,
    )?;

    let selected = THEMES[selection].name;

    // Check if global config exists
    let config_path = global_config_path();

    if !config_path.exists() {
        warning_msg("Global config does not exist.");
        println!();
        print!("  ");
        aura_text(
            "Run 'openzed-git config' and create global config first.",
            TEXT_MUTED,
        );
        println!();
        return Ok(());
    }

    // Read current config
    let content = std::fs::read_to_string(&config_path)?;

    // Update theme line
    let new_content = if content.contains("theme = ") {
        content
            .lines()
            .map(|line| {
                if line.trim().starts_with("theme = ") {
                    format!("theme = \"{}\"", selected)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        // Add theme if not present
        format!("{}\ntheme = \"{}\"", content.trim(), selected)
    };

    std::fs::write(&config_path, new_content)?;

    success(&format!("Theme set to '{}'.", selected));
    println!();
    print!("  ");
    aura_text("Restart OpenZed Git to see changes.", TEXT_MUTED);
    println!();

    Ok(())
}
