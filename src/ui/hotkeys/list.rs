use ratatui::{
    style::Color,
    widgets::{Borders, List},
};

use crate::utils::create_list;

pub fn create_hotkey_list() -> List<'static> {
    let shortcuts = vec![
        "[e] - edit".to_string(),
        "[f] - search".to_string(),
        "[1] - switch to saved".to_string(),
    ];

    let list = create_list(
        " Hotkeys ".to_string(),
        shortcuts,
        Color::LightBlue,
        Color::Gray,
        Borders::ALL,
    );
    list
}
