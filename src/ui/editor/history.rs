use crate::utils::create_list;
use ratatui::{
    style::Color,
    widgets::{Borders, List},
};

pub fn create_editor_history_list<'a>() -> List<'a> {
    let shortcuts = vec![
        format!(
            "{} - {} - [{}]",
            "[200]", "https://localhost:8080/test", "HH:MM:SS / DD.MM.YYYY"
        ),
        format!(
            "{} - {} - [{}]",
            "[200]", "https://localhost:8080/test", "HH:MM:SS / DD.MM.YYYY"
        ),
    ];

    let list = create_list(
        " Request History ".to_string(),
        shortcuts,
        Color::LightGreen,
        Color::Gray,
        Borders::TOP | Borders::BOTTOM,
    );
    list
}
