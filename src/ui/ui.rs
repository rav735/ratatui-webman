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
    let mut constr = vec![
        Constraint::Length(35),
        Constraint::Length(30),
        Constraint::Percentage(100),
    ];

    if db.panel_enabled {
        constr.push(Constraint::Percentage(20));
    }

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constr)
        .split(frame.area());

    frame.render_widget(&request_list.list, layout[0]);
    create_editor(frame, layout[2], &area.area);
    frame.render_widget(hotkeys.create_hotkeys_panel(), layout[1]);

    if db.panel_enabled {
        db.create_debugger_panel(frame, layout[3]);
    }

    let mut c = 1;
    if db.panel_enabled {
        for debug_v in db.categories.clone().iter() {
            hotkeys.add(
                "F".to_string() + &c.to_string(),
                "Debug: ".to_string() + &debug_v.to_string(),
                "Debugger".to_string(),
            );
            c += 1;
        }
    };
}
