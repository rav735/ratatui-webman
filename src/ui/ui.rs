use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

// General
use crate::{
    ui::{
        editor::{EditorTextArea, create_editor},
        hotkeys::Hotkeys,
        saved_requests::SavedRequestList,
    },
    utils::Debugger,
};

pub fn ui(
    frame: &mut Frame,
    request_list: &SavedRequestList,
    area: &EditorTextArea,
    hotkeys: &mut Hotkeys,
    db: &Debugger,
) {
    // Create the layout sections.
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(35),
            Constraint::Length(30),
            Constraint::Percentage(50),
            Constraint::Percentage(20),
        ])
        .split(frame.area());

    frame.render_widget(&request_list.list, layout[0]);
    create_editor(frame, layout[2], &area.area);

    frame.render_widget(hotkeys.create_hotkeys_panel(), layout[1]);
    db.create_debugger_panel(frame, layout[3]);
}
