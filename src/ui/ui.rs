use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};
use tui_textarea::TextArea;

// General
use crate::app::App;

// Editor Ui - Elements
use crate::ui::{
    editor::create_editor, hotkeys::create_hotkey_list, saved_requests::create_saved_list,
};

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
