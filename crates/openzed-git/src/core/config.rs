#![allow(unused)]

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// OpenZed Git configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenZedConfig {
    pub default_branch: String,
    pub default_visibility: String,
    pub auto_open_browser: bool,
    pub theme: String,
    pub commit_style: String,
    pub default_pr_base: String,
    #[serde(default)]
    pub remote: String,
    pub github_owner: Option<String>,
}

impl Default for OpenZedConfig {
    fn default() -> Self {
        Self {
            default_branch: "main".to_string(),
            default_visibility: "public".to_string(),
            auto_open_browser: true,
            theme: "aura-dark".to_string(),
            commit_style: "conventional".to_string(),
            default_pr_base: "main".to_string(),
            remote: "origin".to_string(),
            github_owner: None,
        }
    }
}

/// Get the config directory
pub fn config_dir() -> PathBuf {
    let dirs =
        ProjectDirs::from("com", "openzed", "git").expect("Failed to get project directories");
    dirs.config_dir().to_path_buf()
}

/// Get the global config path ~/.config/openzed-git/config.toml
pub fn global_config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("No HOME directory");
    PathBuf::from(home)
        .join(".config")
        .join("openzed-git")
        .join("config.toml")
}

/// Get the project config path .openzed-git.toml
pub fn project_config_path() -> PathBuf {
    PathBuf::from(".openzed-git.toml")
}

/// Load config with project config overriding global config
pub fn load_config() -> anyhow::Result<OpenZedConfig> {
    let mut config = OpenZedConfig::default();

    // Load global config if exists
    let global_path = global_config_path();
    if global_path.exists() {
        match std::fs::read_to_string(&global_path) {
            Ok(content) => match toml::from_str::<OpenZedConfig>(&content) {
                Ok(global_cfg) => {
                    config = global_cfg;
                }
                Err(e) => {
                    eprintln!("Warning: Invalid global config: {}. Using defaults.", e);
                }
            },
            Err(e) => {
                eprintln!(
                    "Warning: Could not read global config: {}. Using defaults.",
                    e
                );
            }
        }
    }

    // Load and override with project config if exists
    let project_path = project_config_path();
    if project_path.exists() {
        match std::fs::read_to_string(&project_path) {
            Ok(content) => {
                match toml::from_str::<OpenZedConfig>(&content) {
                    Ok(project_cfg) => {
                        // Override only specified fields
                        if project_cfg.default_branch != "main" {
                            config.default_branch = project_cfg.default_branch;
                        }
                        if !project_cfg.default_visibility.is_empty() {
                            config.default_visibility = project_cfg.default_visibility;
                        }
                        // booleans use their default false if not explicitly set in project
                        config.auto_open_browser = project_cfg.auto_open_browser;
                        if !project_cfg.theme.is_empty() && project_cfg.theme != "aura-dark" {
                            config.theme = project_cfg.theme;
                        }
                        if !project_cfg.commit_style.is_empty() {
                            config.commit_style = project_cfg.commit_style;
                        }
                        if !project_cfg.default_pr_base.is_empty() {
                            config.default_pr_base = project_cfg.default_pr_base;
                        }
                        if !project_cfg.remote.is_empty() {
                            config.remote = project_cfg.remote;
                        }
                        if project_cfg.github_owner.is_some() {
                            config.github_owner = project_cfg.github_owner;
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Invalid project config: {}. Ignoring.", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not read project config: {}. Ignoring.", e);
            }
        }
    }

    Ok(config)
}

/// Create global config file
pub fn create_global_config() -> anyhow::Result<()> {
    let path = global_config_path();

    // Check if exists
    if path.exists() {
        anyhow::bail!(
            "Global config already exists at {}. Will not overwrite.",
            path.display()
        );
    }

    // Create directory
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let default_config = r#"# OpenZed Git Global Configuration
# This file is located at ~/.config/openzed-git/config.toml

default_branch = "main"
default_visibility = "public"
auto_open_browser = true
theme = "aura-dark"
commit_style = "conventional"
default_pr_base = "main"
remote = "origin"
"#;

    std::fs::write(&path, default_config)?;
    Ok(())
}

/// Create project config file
pub fn create_project_config() -> anyhow::Result<()> {
    let path = project_config_path();

    // Check if exists
    if path.exists() {
        anyhow::bail!(
            "Project config already exists at {}. Will not overwrite.",
            path.display()
        );
    }

    let default_config = r#"# OpenZed Git Project Configuration
# This file is located at .openzed-git.toml in your project root
# Values here override global config

project_name = "my-project"
default_branch = "main"
remote = "origin"
"#;

    std::fs::write(&path, default_config)?;
    Ok(())
}

/// Get the Zed keymap path
pub fn zed_keymap_path() -> PathBuf {
    let home = std::env::var("HOME").expect("No HOME directory");
    PathBuf::from(home)
        .join(".config")
        .join("zed")
        .join("keymap.json")
}

/// Get suggested keybindings JSON
pub fn suggested_keybindings() -> String {
    r#"[
  {
    "context": "Workspace",
    "bindings": {
      "cmd-alt-g m": ["task::Spawn", { "task_name": "OpenZed Git: Menu" }],
      "cmd-alt-g s": ["task::Spawn", { "task_name": "OpenZed Git: Status+" }],
      "cmd-alt-g g": ["task::Spawn", { "task_name": "OpenZed Git: Git Graph" }],
      "cmd-alt-g c": ["task::Spawn", { "task_name": "OpenZed Git: Commit Assistant" }],
      "cmd-alt-g p": ["task::Spawn", { "task_name": "OpenZed Git: Publish to GitHub" }],
      "cmd-alt-g r": ["task::Spawn", { "task_name": "OpenZed Git: Pull Requests" }]
    }
  }
]"#
    .to_string()
}
