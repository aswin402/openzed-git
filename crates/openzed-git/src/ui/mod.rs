#![allow(unused)]

pub mod aura;
pub mod banner;
pub mod prompts;

use crate::ui::aura::aura::{CHECK, CYAN, GREEN, RESET, SURFACE, TEXT_DIM};
use std::io::{self, Write};

/// Loading animation frames
pub const LOADING_FRAMES: [&str; 8] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];

/// Animated spinner that displays while a process is running
pub struct Spinner {
    message: String,
    done_message: String,
    frames: Vec<String>,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            done_message: String::new(),
            frames: LOADING_FRAMES.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_done_message(mut self, done_message: &str) -> Self {
        self.done_message = done_message.to_string();
        self
    }

    pub fn spin(&self, frame: usize) {
        let frame_str = &self.frames[frame % self.frames.len()];
        print!("\r{} {} {}", CYAN, frame_str, RESET);
        print!(" {}", self.message);
        print!("{}", " ".repeat(20));
        io::stdout().flush().ok();
    }

    pub fn finish(&self) {
        print!("\r{} {} {}", CHECK, "✓", RESET);
        print!(" {}", self.done_message);
        println!();
    }
}

/// Progress bar for operations with known progress
pub struct ProgressBar {
    total: usize,
    current: usize,
    width: usize,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: 0,
            width: 30,
        }
    }

    pub fn update(&mut self, current: usize) {
        self.current = current;
    }

    pub fn render(&self) {
        let filled = (self.current * self.width) / self.total;
        let empty = self.width - filled;

        print!("\r{} [", SURFACE);
        print!("{}{}", GREEN, "█".repeat(filled));
        print!("{}", TEXT_DIM);
        print!("{}", "░".repeat(empty));
        print!("{}] {}%", SURFACE, (self.current * 100) / self.total);
        io::stdout().flush().ok();
    }

    pub fn finish(&self) {
        print!("\r{} [", SURFACE);
        print!("{}{}", GREEN, "█".repeat(self.width));
        print!("{}] 100%", SURFACE);
        println!();
    }
}
