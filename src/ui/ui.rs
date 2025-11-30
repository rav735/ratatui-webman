use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::List,
};
use tui_textarea::TextArea;

// General
use crate::{EditorState, ui::{debug::create_debuger_panel, editor::create_editor, hotkeys::create_hotkey_list}, utils::DebugValues};

pub fn ui(frame: &mut Frame, request_list: &List, area: &TextArea, db : &DebugValues) {
    // Create the layout sections.
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(35),
            Constraint::Length(30),
            Constraint::Percentage(35),
            Constraint::Percentage(35)
        ])
        .split(frame.area());

    frame.render_widget(request_list, layout[0]);
    create_editor(frame, layout[2], area);

    let hotkey_list = create_hotkey_list();
    frame.render_widget(hotkey_list, layout[1]);

    
    frame.render_widget(db.create_debuger_panel(), layout[3]);
}
