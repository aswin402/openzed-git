#![allow(unused)]

use std::fmt::Display;
use std::io::{self, Write};

/// Modern Theme - White, Green, Pink colors
pub mod aura {
    // Text colors
    pub const TEXT: &str = "\x1b[38;2;255;255;255m"; // White
    pub const TEXT_MUTED: &str = "\x1b[38;2;180;180;180m"; // Gray
    pub const TEXT_DIM: &str = "\x1b[38;2;120;120;120m"; // Dim gray

    // Accent colors
    pub const GREEN: &str = "\x1b[38;2;0;255;135m"; // Bright green
    pub const PINK: &str = "\x1b[38;2;255;105;180m"; // Hot pink
    pub const LAVENDER: &str = "\x1b[38;2;255;182;193m"; // Light pink (moved from purple)
    pub const BLUE: &str = "\x1b[38;2;100;200;255m"; // Sky blue
    pub const YELLOW: &str = "\x1b[38;2;255;255;100m"; // Yellow
    pub const ORANGE: &str = "\x1b[38;2;255;165;80m"; // Orange

    // Status colors
    pub const SUCCESS: &str = "\x1b[38;2;0;255;135m"; // Bright green
    pub const WARNING: &str = "\x1b[38;2;255;255;100m"; // Yellow
    pub const ERROR: &str = "\x1b[38;2;255;80;80m"; // Red
    pub const INFO: &str = "\x1b[38;2;100;200;255m"; // Blue

    // UI Elements
    pub const BORDER: &str = "\x1b[38;2;80;80;80m"; // Gray border
    pub const SURFACE: &str = "\x1b[48;2;30;30;30m"; // Dark surface

    // Icons
    pub const CHECK: &str = "\x1b[38;2;0;255;135m✓\x1b[0m";
    pub const CROSS: &str = "\x1b[38;2;255;80;80m✗\x1b[0m";
    pub const WARN_ICON: &str = "\x1b[38;2;255;255;100m⚠\x1b[0m";
    pub const ARROW: &str = "\x1b[38;2;255;105;180m›\x1b[0m";
    pub const STAGED: &str = "\x1b[38;2;0;255;135m●\x1b[0m";
    pub const MODIFIED: &str = "\x1b[38;2;255;255;100m◎\x1b[0m";
    pub const DELETED: &str = "\x1b[38;2;255;80;80m◼\x1b[0m";
    pub const UNTRACKED: &str = "\x1b[38;2;180;180;180m○\x1b[0m";
    pub const BRANCH: &str = "\x1b[38;2;255;105;180m⎇\x1b[0m";
    pub const COMMIT: &str = "\x1b[38;2;0;255;135m•\x1b[0m";
    pub const LINE: &str = "\x1b[38;2;80;80;80m│\x1b[0m";

    // Reset
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
}

/// Print colored text
pub fn aura_text(text: &str, color: &str) {
    print!("{}{}{}", color, text, aura::RESET);
}

/// Print bold text
pub fn aura_bold(text: &str, color: &str) {
    print!("{}{}{}{}", aura::BOLD, color, text, aura::RESET);
}

pub fn separator(title: &str) {
    let width = 50;
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

pub fn item<K: Display, V: Display>(key: K, value: V) {
    print!("  {} ", aura::ARROW);
    aura_text(&format!("{}:", key), aura::TEXT_MUTED);
    aura_text(&value.to_string(), aura::TEXT);
    println!();
}

pub fn success(msg: &str) {
    print!("{} ", aura::CHECK);
    aura_text(msg, aura::GREEN);
}

pub fn error_msg(msg: &str) {
    print!("{} ", aura::CROSS);
    aura_text(msg, aura::ERROR);
}

pub fn warning_msg(msg: &str) {
    print!("{} ", aura::WARN_ICON);
    aura_text(msg, aura::WARNING);
}

pub fn info_msg(msg: &str) {
    print!("{} ", aura::INFO);
    aura_text(msg, aura::INFO);
}

pub fn heading(msg: &str) {
    aura_bold(msg, aura::PINK);
}

pub fn subheading(msg: &str) {
    aura_text(msg, aura::TEXT_MUTED);
}

pub fn dim(msg: &str) {
    aura_text(msg, aura::TEXT_DIM);
}

pub fn empty() {
    println!();
}
