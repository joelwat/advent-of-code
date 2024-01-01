use console::Term;
use dialoguer::{
    theme::ColorfulTheme,
    Select,
};

use super::app_errors::AppError;

pub type RunFn = fn() -> Result<(), AppError>;

pub fn ask_user(run1: RunFn, run2: RunFn) -> Result<RunFn, AppError> {
    let items = vec!["part1", "part2", "part3"];

    let input = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?
        .unwrap();

    let selection = items.get(input).ok_or(AppError::GenericError(
        "Error getting user input".to_string(),
    ))?;

    if *selection == "part1" {
        return Ok(run1);
    }

    Ok(run2)
}
