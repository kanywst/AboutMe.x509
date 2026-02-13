use crate::terminal::args::Args;
use anyhow::Result;
use dialoguer::{Input, theme::ColorfulTheme};

/// Enriches the provided Args with interactive input if mandatory fields are missing.
pub fn run_wizard(args: &mut Args) -> Result<()> {
    let theme = ColorfulTheme::default();

    if args.name.is_none() {
        args.name = Some(Input::with_theme(&theme).with_prompt("Name").interact_text()?);

        args.hometown = Some(
            Input::with_theme(&theme)
                .with_prompt("Hometown [Optional]")
                .allow_empty(true)
                .interact_text()?,
        );

        args.age = Some(
            Input::with_theme(&theme)
                .with_prompt("Age [Optional]")
                .allow_empty(true)
                .interact_text()?,
        );

        args.occupation = Some(
            Input::with_theme(&theme)
                .with_prompt("Occupation [Optional]")
                .allow_empty(true)
                .interact_text()?,
        );

        args.hobby = Some(
            Input::with_theme(&theme)
                .with_prompt("Hobby [Optional]")
                .allow_empty(true)
                .interact_text()?,
        );

        args.motto = Some(
            Input::with_theme(&theme)
                .with_prompt("Motto [Optional]")
                .allow_empty(true)
                .interact_text()?,
        );
    }
    Ok(())
}
