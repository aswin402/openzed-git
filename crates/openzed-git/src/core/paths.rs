#![allow(unused)]

pub fn zed_tasks_path() -> std::path::PathBuf {
    let dirs = directories::ProjectDirs::from("com", "openzed", "git")
        .expect("Failed to get project directories");

    match dirs.config_dir().to_str() {
        Some("") => {
            // Fallback for Linux/macOS
            let home = std::env::var("HOME").expect("No HOME directory");
            std::path::PathBuf::from(home)
                .join(".config")
                .join("zed")
                .join("tasks.json")
        }
        _ => dirs.config_dir().join("zed").join("tasks.json"),
    }
}

pub fn default_zed_tasks_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").expect("No HOME directory");
    std::path::PathBuf::from(home)
        .join(".config")
        .join("zed")
        .join("tasks.json")
}

/// Get project-specific log directory (.openzed-git in current dir or git root)
pub fn project_log_dir() -> std::path::PathBuf {
    // Check if we're in a git repo
    let git_repo = std::path::Path::new(".git");
    if git_repo.exists() {
        return std::path::PathBuf::from(".openzed-git");
    }

    // Fallback to global config dir
    let home = std::env::var("HOME").unwrap_or_default();
    std::path::PathBuf::from(home)
        .join(".config")
        .join("openzed-git")
}
