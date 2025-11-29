use crate::{app::App, ui::editor::{header::create_editor_header, history::create_editor_history_list}};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};
use tui_textarea::TextArea;

pub fn create_editor<'a>(app : &App, frame: &mut Frame, layout: Rect, area: &TextArea) {
    let editor_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(layout);

    let editor_header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(editor_layout[0]);

    let (project, name) = create_editor_header();
    frame.render_widget(project, editor_header_layout[0]);
    frame.render_widget(name, editor_header_layout[1]);

    let editor_area = area;
    frame.render_widget(editor_area, editor_layout[1]);

    let history_list = create_editor_history_list(app);
    frame.render_widget(history_list, editor_layout[2]);
}
