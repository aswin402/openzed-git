#![allow(unused)]

use std::fmt::Display;

/// Aura Dark Theme - For OpenZed Git TUI
pub mod aura {
    // Background colors
    pub const BG: &str = "\x1b[48;2;28;27;34m"; // #1C1B22
    pub const SURFACE: &str = "\x1b[48;2;33;34;44m"; // #21222C

    // Text colors
    pub const TEXT: &str = "\x1b[38;2;248;248;242m"; // #F8F8F2
    pub const TEXT_MUTED: &str = "\x1b[38;2;98;114;164m"; // #6272A4
    pub const TEXT_DIM: &str = "\x1b[38;2;109;106;124m"; // #6D6A7C

    // Accent colors
    pub const PURPLE: &str = "\x1b[38;2;162;119;255m"; // #A277FF
    pub const PINK: &str = "\x1b[38;2;255;113;231m"; // #FF71E7
    pub const CYAN: &str = "\x1b[38;2;99;209;169m"; // #63D1A9
    pub const GREEN: &str = "\x1b[38;2;157;255;101m"; // #9DFF65
    pub const YELLOW: &str = "\x1b[38;2;255;228;106m"; // #FFE46A
    pub const RED: &str = "\x1b[38;2;255;103;103m"; // #FF6767
    pub const WHITE: &str = "\x1b[38;2;255;255;255m"; // #FFFFFF

    // Status colors
    pub const SUCCESS: &str = "\x1b[38;2;157;255;101m"; // #9DFF65
    pub const WARNING: &str = "\x1b[38;2;255;228;106m"; // #FFE46A
    pub const ERROR: &str = "\x1b[38;2;255;103;103m"; // #FF6767
    pub const INFO: &str = "\x1b[38;2;99;209;169m"; // #63D1A9

    // UI Elements
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";

    // Icons
    pub const CHECK: &str = "\x1b[38;2;157;255;101m✓\x1b[0m";
    pub const CROSS: &str = "\x1b[38;2;255;103;103m✗\x1b[0m";
    pub const WARN: &str = "\x1b[38;2;255;228;106m⚠\x1b[0m";
    pub const ARROW: &str = "\x1b[38;2;255;113;231m›\x1b[0m";
    pub const SELECTED: &str = "\x1b[38;2;99;209;169m▸\x1b[0m"; // CYAN
    pub const BULLET: &str = "\x1b[38;2;99;209;169m●\x1b[0m"; // CYAN
    pub const STAGED: &str = "\x1b[38;2;157;255;101m●\x1b[0m";
    pub const MODIFIED: &str = "\x1b[38;2;255;228;106m◎\x1b[0m";
    pub const DELETED: &str = "\x1b[38;2;255;103;103m◼\x1b[0m";
    pub const UNTRACKED: &str = "\x1b[38;2;98;114;164m○\x1b[0m";

    // Graph symbols
    pub const GRAPH_COMMIT: &str = "\x1b[38;2;99;209;169m●\x1b[0m";
    pub const GRAPH_LINE: &str = "\x1b[38;2;99;209;169m│\x1b[0m";
    pub const GRAPH_BRANCH: &str = "\x1b[38;2;162;119;255m⎇\x1b[0m";
    pub const GRAPH_ARROW: &str = "\x1b[38;2;255;113;231m→\x1b[0m";
}

/// Print colored text
pub fn aura_text(text: &str, color: &str) {
    print!("{}{}{}", color, text, aura::RESET);
}

/// Print bold text
pub fn aura_bold(text: &str, color: &str) {
    print!("{}{}{}{}", aura::BOLD, color, text, aura::RESET);
}

/// Print with trailing space
pub fn aura_print(text: &str, color: &str) {
    print!("{}{}{} ", color, text, aura::RESET);
}

/// Print separator/header
pub fn separator(title: &str) {
    let width = 54;
    let padding = (width - title.len()) / 2;
    println!();
    println!("{}─{}─", aura::TEXT_DIM, "─".repeat(width));
    print!("{}│", aura::SURFACE);
    print!("{}", " ".repeat(padding));
    aura_bold(title, aura::PINK);
    println!();
    println!("{}─{}─", aura::TEXT_DIM, "─".repeat(width));
    println!();
}

/// Print item key value
pub fn item<K: Display, V: Display>(key: K, value: V) {
    print!("  {} ", aura::ARROW);
    aura_text(&format!("{}:", key), aura::TEXT_MUTED);
    aura_text(&value.to_string(), aura::TEXT);
    println!();
}

/// Print empty line
pub fn empty() {
    println!();
}

/// Print success message
pub fn success(msg: &str) {
    print!("  ");
    aura_text(aura::CHECK.trim(), aura::GREEN);
    print!(" {} ", msg);
    println!();
}

/// Print error message
pub fn error_msg(msg: &str) {
    print!("  ");
    aura_text(aura::CROSS.trim(), aura::RED);
    print!(" {} ", msg);
    println!();
}

/// Print warning message
pub fn warning_msg(msg: &str) {
    print!("  ");
    aura_text(aura::WARN.trim(), aura::YELLOW);
    print!(" {} ", msg);
    println!();
}

/// Print info message
pub fn info_msg(msg: &str) {
    print!("  ");
    aura_text(aura::BULLET.trim(), aura::CYAN);
    print!(" {} ", msg);
    println!();
}

/// Print status line: label + ok/error icon
pub fn status_line(label: &str, ok: bool) {
    print!("  ");
    aura_text(label, aura::TEXT);
    if ok {
        print!("... ");
        aura_text(aura::CHECK.trim(), aura::GREEN);
    } else {
        print!("... ");
        aura_text(aura::CROSS.trim(), aura::RED);
    }
    println!();
}

/// Print labeled value
pub fn labeled_value(label: &str, value: &str) {
    print!("  ");
    aura_text(label, aura::TEXT_MUTED);
    print!(" ");
    aura_text(value, aura::TEXT);
    println!();
}

/// Print with arrow prefix
pub fn with_arrow(label: &str) {
    print!("  ");
    aura_print(aura::ARROW.trim(), aura::PINK);
    aura_text(label, aura::TEXT_MUTED);
    println!();
}

/// Print prompt label
pub fn prompt_label(label: &str) {
    print!("  ");
    aura_text(label, aura::TEXT);
}

/// Print default value in brackets
pub fn default_value(value: &str) -> String {
    format!("[{}]", value)
}

/// Print prompt with default value
pub fn prompt_with_default(label: &str, default: &str) {
    print!("  ");
    aura_text(label, aura::TEXT);
    print!(" ");
    aura_text(&default_value(default), aura::TEXT_DIM);
    print!(": ");
}

/// Print link/url in purple
pub fn link(url: &str) {
    print!("  ");
    aura_text("GitHub:", aura::TEXT_MUTED);
    print!(" ");
    aura_text(url, aura::PURPLE);
    println!();
}
