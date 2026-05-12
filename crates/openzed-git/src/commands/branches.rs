use crate::core::git::GitInfo;
use crate::ui::aura::separator;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Branches");

    let branches = GitInfo::branches()?;
    println!("{}", branches);

    Ok(())
}
