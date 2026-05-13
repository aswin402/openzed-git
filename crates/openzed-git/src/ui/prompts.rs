use anyhow::{Context, Result};
use dialoguer::Input;
use dialoguer::Select;
use dialoguer::MultiSelect;

/// Wrapper to check for terminal before interactive operations
fn check_terminal() -> Result<()> {
    if !atty::is(atty::Stream::Stdin) {
        anyhow::bail!("Interactive input requires a terminal. Run this command in a terminal.");
    }
    Ok(())
}

pub fn input(prompt: &str) -> Result<String> {
    check_terminal()?;
    let result = Input::new()
        .with_prompt(prompt)
        .interact()
        .context("Interactive input failed")?;
    Ok(result)
}

pub fn input_with_default(prompt: &str, default: &str) -> Result<String> {
    check_terminal()?;
    let result = Input::new()
        .with_prompt(prompt)
        .default(default.to_string())
        .interact()
        .context("Interactive input failed")?;
    Ok(result)
}

pub fn select(prompt: &str, items: &[&str]) -> Result<usize> {
    check_terminal()?;
    let selection = Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()
        .context("Selection failed")?;
    Ok(selection)
}

pub fn select_with_default(prompt: &str, items: &[&str], default: usize) -> Result<usize> {
    check_terminal()?;
    let selection = Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(default)
        .interact()
        .context("Selection failed")?;
    Ok(selection)
}

pub fn multiselect(prompt: &str, items: &[String]) -> Result<Vec<usize>> {
    check_terminal()?;
    let selections = MultiSelect::new()
        .with_prompt(prompt)
        .items(items)
        .interact()
        .context("Selection failed")?;
    Ok(selections)
}
