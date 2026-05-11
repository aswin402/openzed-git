use crate::core::{parser::ZedTasks, paths, tasks};
use crate::ui::{colors::*, output::separator};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Install Zed Tasks");

    let tasks_path = paths::default_zed_tasks_path();

    println!("  Loading tasks from: {:?}", tasks_path);

    let mut zed_tasks = ZedTasks::load(&tasks_path)?;
    let openzed_tasks = tasks::get_openzed_git_tasks();

    println!("  Found {} existing tasks", zed_tasks.tasks().len());

    zed_tasks.merge_tasks(openzed_tasks);

    println!("  Merged to {} total tasks", zed_tasks.tasks().len());

    // Ensure parent directory exists
    if let Some(parent) = tasks_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    zed_tasks.save(&tasks_path)?;

    println!();
    println!("  {} OpenZed Git tasks installed.", CHECK);
    println!();
    println!("  Use in Zed:");
    println!("  1. Open Command Palette");
    println!("  2. Run `task: spawn`");
    println!("  3. Select `OpenZed Git: <command>`");

    Ok(())
}
