use crate::core::git::GitInfo;
use crate::ui::aura::separator;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Git Graph");

    let graph = GitInfo::log_simple()?;
    println!("{}", graph);

    Ok(())
}
