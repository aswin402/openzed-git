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
