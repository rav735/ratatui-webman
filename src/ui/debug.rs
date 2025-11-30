use crate::{EditorState, utils::create_list};
use ratatui::{
    style::Color,
    widgets::{Borders, List},
};

pub fn create_debuger_panel<'a>(state : &EditorState, last_sel: &str, sel: &str) -> List<'a> {
    let shortcuts = vec![
        format!("State {state:?} "),
        "Last - ".to_string() + last_sel,
        "Selected - ".to_string() + sel,
    ];

    let list = create_list(
        " Debugger ".to_string(),
        shortcuts,
        Color::LightYellow,
        Color::LightCyan,
        Borders::ALL,
    );
    list
}
