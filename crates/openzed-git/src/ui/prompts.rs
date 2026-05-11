use dialoguer::Input;

pub fn input(prompt: &str) -> anyhow::Result<String> {
    let result = Input::new().with_prompt(prompt).interact()?;

    Ok(result)
}

pub fn input_with_default(prompt: &str, default: &str) -> anyhow::Result<String> {
    let result = Input::new()
        .with_prompt(prompt)
        .default(default.to_string())
        .interact()?;

    Ok(result)
}
