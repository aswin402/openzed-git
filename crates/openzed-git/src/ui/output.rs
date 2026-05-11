use std::fmt::Display;

pub fn separator(title: &str) {
    let width = 60;
    let padding = (width - title.len()) / 2;
    println!();
    println!("{}", "─".repeat(width));
    println!("{:padding$}{}", "", title, padding = padding);
    println!("{}", "─".repeat(width));
    println!();
}

pub fn item<K: Display, V: Display>(key: K, value: V) {
    println!("  {}: {}", key, value);
}

pub fn empty() {
    println!();
}
