use colored::Colorize;

pub const CHECK: &str = "✅";
pub const WARNING: &str = "⚠️";
pub const CROSS: &str = "❌";
pub const STAGED: &str = "📝";
pub const UNTRACKED: &str = "➕";
pub const MODIFIED: &str = "📝";
pub const DELETED: &str = "🗑️";
pub const RENAMED: &str = "📛";

pub fn success(msg: &str) -> ColoredString {
    msg.green()
}

pub fn warning(msg: &str) -> ColoredString {
    msg.yellow()
}

pub fn error(msg: &str) -> ColoredString {
    msg.red()
}

pub fn info(msg: &str) -> ColoredString {
    msg.cyan()
}

pub fn heading(msg: &str) -> ColoredString {
    msg.bold().bright_blue()
}

pub fn prompt(msg: &str) -> ColoredString {
    msg.bright_magenta()
}

use colored::ColoredString;
