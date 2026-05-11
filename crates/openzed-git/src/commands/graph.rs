use crate::core::git::GitInfo;
use crate::ui::output::separator;
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Git Graph");

    let graph = GitInfo::log_graph()?;
    println!("{}", graph);

    Ok(())
}
