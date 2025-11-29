use crate::{app::App, utils::create_list};
use ratatui::{
    style::Color,
    widgets::{Borders, List},
};

pub fn create_saved_list(app: &App) -> List<'static> {
    let shortcuts = vec![
        format!("app.current_screen : {:#?}", app.current_screen).to_string(),
        format!("app.currently_editing : {:#?}", app.currently_editing).to_string(),
    ];
    let list = create_list(
        " [1] - Saved Requests ".to_string(),
        shortcuts,
        Color::DarkGray,
        Color::Gray,
        Borders::ALL,
    );
    list
}
