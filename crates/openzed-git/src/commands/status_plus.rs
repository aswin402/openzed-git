use crate::core::git::GitInfo;
use crate::ui::{
    colors::*,
    output::{empty, item, separator},
};
use anyhow::Result;

pub fn run() -> Result<()> {
    separator("OpenZed Git: Status+");

    let status = GitInfo::status()?;
    let info = GitInfo::get()?;
    let upstream = GitInfo::upstream().ok().flatten();

    // Parse status output
    let lines: Vec<&str> = status.lines().collect();

    // First line is branch info
    if let Some(branch_line) = lines.first() {
        let branch_info = branch_line.trim();
        item("Status", branch_info);
    }

    if let Some(ref upstream) = upstream {
        item("Upstream", upstream);
    }

    empty();

    // Counters
    let mut staged = 0;
    let mut modified = 0;
    let mut deleted = 0;
    let _renamed = 0;
    let mut untracked = 0;

    for line in &lines[1..] {
        if line.len() < 2 {
            continue;
        }
        let index_status = line.chars().next().unwrap_or(' ');
        let worktree_status = line.chars().nth(1).unwrap_or(' ');

        match index_status {
            'A' | 'M' | 'R' | 'C' => staged += 1,
            'D' => {
                staged += 1;
                deleted += 1;
            }
            _ => {}
        }

        match worktree_status {
            'M' => modified += 1,
            'D' => deleted += 1,
            '?' => untracked += 1,
            _ => {}
        }
    }

    if staged > 0 {
        println!("  {} Staged: {}", STAGED, staged);
    }
    if modified > 0 {
        println!("  {} Modified: {}", MODIFIED, modified);
    }
    if deleted > 0 {
        println!("  {} Deleted: {}", DELETED, deleted);
    }
    if untracked > 0 {
        println!("  {} Untracked: {}", UNTRACKED, untracked);
    }

    empty();

    // Show remote
    if let Some(ref remote) = info.remote_origin {
        println!("  {} Remote: {}", CHECK, remote);
    } else {
        println!("  {} No remote origin", WARNING);
    }

    println!();

    Ok(())
}
