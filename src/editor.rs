use crate::{app::App, list::create_list};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, Padding, Paragraph},
};
use tui_textarea::TextArea;

pub fn create_editor<'a>(frame: &mut Frame, layout: Rect, area: &TextArea) {
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

    let history_list = create_editor_history_list();
    frame.render_widget(history_list, editor_layout[2]);
}

pub fn create_editor_area<'a>(app: &App) -> TextArea<'a> {
    let style = Style::default().bg(Color::DarkGray);

    let mut textarea = TextArea::from(serde_json::to_string_pretty(&app.value).unwrap().lines());
    textarea.set_line_number_style(style);
    textarea.set_block(
        Block::default()
            .borders(Borders::TOP)
            .style(Style::default())
            .title(Line::from("Request Body").centered())
            .padding(Padding {
                left: 3,
                right: 0,
                top: 1,
                bottom: 1,
            }),
    );
    textarea
}

fn create_editor_header<'a>() -> (Paragraph<'a>, Paragraph<'a>) {
    let project = create_paragraph(
        "Project".to_string(),
        "Name".to_string(),
        Color::Gray,
        Borders::TOP,
        Alignment::Center,
    );
    let name = create_paragraph(
        "Request".to_string(),
        "Name".to_string(),
        Color::Gray,
        Borders::TOP | Borders::LEFT,
        Alignment::Left,
    );

    (project, name)
}

fn create_paragraph<'a>(
    titel: String,
    content: String,
    color: Color,
    border: Borders,
    content_aligment: Alignment,
) -> Paragraph<'a> {
    let project = Paragraph::new(Text::styled(content, Style::default().fg(color)))
        .alignment(content_aligment)
        .block(
            Block::default()
                .borders(border)
                .style(Style::default())
                .title(titel)
                .title_alignment(Alignment::Center),
        );
    project
}

fn create_editor_history_list<'a>() -> List<'a> {
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
        "Request History".to_string(),
        shortcuts,
        Color::LightGreen,
        Color::Gray,
        Borders::TOP | Borders::BOTTOM,
    );
    list
}
