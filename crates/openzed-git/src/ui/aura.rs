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

    // Status color aliases
    pub const WARNING: &str = "\x1b[38;2;255;228;106m"; // #FFE46A (same as YELLOW)
    pub const SUCCESS: &str = "\x1b[38;2;157;255;101m"; // #9DFF65 (same as GREEN)
    pub const ERROR: &str = "\x1b[38;2;255;103;103m"; // #FF6767 (same as RED)
    pub const INFO: &str = "\x1b[38;2;99;209;169m"; // #63D1A9 (same as CYAN)

    // Reset and style
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";

    // Status icons with color baked in
    pub const CHECK: &str = "\x1b[38;2;157;255;101m✓\x1b[0m";
    pub const CROSS: &str = "\x1b[38;2;255;103;103m✗\x1b[0m";
    pub const WARN: &str = "\x1b[38;2;255;228;106m⚠\x1b[0m";
    pub const ARROW: &str = "\x1b[38;2;255;113;231m›\x1b[0m";

    // Graph symbols
    pub const BULLET: &str = "\x1b[38;2;99;209;169m●\x1b[0m"; // CYAN
    pub const LINE: &str = "\x1b[38;2;99;209;169m│\x1b[0m"; // CYAN

    // File status icons
    pub const STAGED: &str = "\x1b[38;2;157;255;101m●\x1b[0m"; // GREEN
    pub const MODIFIED: &str = "\x1b[38;2;255;228;106m◎\x1b[0m"; // YELLOW
    pub const DELETED: &str = "\x1b[38;2;255;103;103m◼\x1b[0m"; // RED
    pub const UNTRACKED: &str = "\x1b[38;2;98;114;164m○\x1b[0m"; // TEXT_MUTED
}

/// Print colored text
pub fn aura_text(text: &str, color: &str) {
    print!("{}{}{}", color, text, aura::RESET);
}

/// Print bold text with color
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

/// Print section header (smaller than separator)
pub fn section(title: &str) {
    print!("  ");
    aura_bold(title, aura::CYAN);
    println!();
}

/// Print bullet point
pub fn bullet(text: &str) {
    print!("  ");
    aura_text(aura::BULLET.trim(), aura::CYAN);
    print!(" ");
    aura_text(text, aura::TEXT);
    println!();
}

/// Print muted/dim text
pub fn muted(text: &str) {
    aura_text(text, aura::TEXT_MUTED);
}

/// Print inline error text
pub fn inline_error(text: &str) {
    aura_text(text, aura::RED);
}

/// Print inline success text
pub fn inline_success(text: &str) {
    aura_text(text, aura::GREEN);
}

/// Print header title (pink + bold)
pub fn print_header(title: &str) {
    println!();
    aura_bold(title, aura::PINK);
    println!();
}

/// Print commit hash in cyan
pub fn print_hash(hash: &str) {
    aura_text(hash, aura::CYAN);
}

/// Print date in muted
pub fn print_date(date: &str) {
    aura_text(date, aura::TEXT_MUTED);
}

/// Print branch refs in purple
pub fn print_refs(refs: &str) {
    aura_text(refs, aura::PURPLE);
}

/// Print message in text color
pub fn print_message(msg: &str) {
    aura_text(msg, aura::TEXT);
}

/// Print graph line
pub fn graph_line() {
    aura_text("│", aura::CYAN);
}

/// Print graph bullet
pub fn graph_bullet() {
    aura_text("●", aura::CYAN);
}
