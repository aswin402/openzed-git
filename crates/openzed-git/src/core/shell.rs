use anyhow::{Context, Result};
use std::process::Command;

pub fn run(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command).args(args).output().context(format!(
        "Failed to run {} {}",
        command,
        args.join(" ")
    ))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {}", stderr)
    }
}

pub fn run_success(command: &str, args: &[&str]) -> Result<bool> {
    let output = Command::new(command).args(args).output().context(format!(
        "Failed to run {} {}",
        command,
        args.join(" ")
    ))?;

    Ok(output.status.success())
}

pub fn confirm(prompt: &str) -> Result<bool> {
    let confirmed = dialoguer::Confirm::new()
        .with_prompt(prompt)
        .default(false)
        .interact()?;

    Ok(confirmed)
}
