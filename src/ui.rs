use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        Borders, List,
    },
};
use tui_textarea::TextArea;

use crate::editor::create_editor;
use crate::list::create_list;

pub fn create_hotkey_list() -> List<'static> {
    let shortcuts = vec!["[e] - edit".to_string(),
        "[f] - search".to_string(),
        "[1] - switch to saved".to_string()];

    let list = create_list("Hotkeys".to_string(), shortcuts, Color::LightBlue, Color::Gray, Borders::ALL);
    list
}

pub fn create_saved_list(app: &App) -> List<'static> {
    let shortcuts = vec![
        format!("app.current_screen : {:#?}", app.current_screen).to_string(),
        format!("app.currently_editing : {:#?}", app.currently_editing).to_string()
        ];
    let list = create_list("[1] - Saved Requests".to_string(), shortcuts, Color::DarkGray, Color::Gray, Borders::ALL);
    list
}

pub fn ui(frame: &mut Frame, app: &App, area: &TextArea) {
    // Create the layout sections.
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(50),
            Constraint::Percentage(70),
            Constraint::Length(30),
        ])
        .split(frame.area());

    let list = create_saved_list(app);
    frame.render_widget(list, layout[0]);


    create_editor(frame, layout[1], area);

    let hotkey_list = create_hotkey_list();
    frame.render_widget(hotkey_list, layout[2]);
}