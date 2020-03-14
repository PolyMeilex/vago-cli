use dialoguer::{select, theme::ColorfulTheme};

pub fn run(title: &str, data: &[String]) -> Option<usize> {
    if data.is_empty() {
        return None;
    }
    let theme = ColorfulTheme::default();

    let id = select::Select::with_theme(&theme)
        .with_prompt(title)
        .default(0)
        .items(&data[..])
        .interact();

    if let Ok(id) = id {
        Some(id)
    } else {
        None
    }
}
