use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn create_editor_header<'a>() -> (Paragraph<'a>, Paragraph<'a>) {
    let project = create_paragraph(
        " Project ".to_string(),
        "Name".to_string(),
        Color::Gray,
        Borders::TOP,
        Alignment::Center,
    );
    let name = create_paragraph(
        " Request ".to_string(),
        "Name".to_string(),
        Color::Gray,
        Borders::TOP | Borders::LEFT | Borders::RIGHT,
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
                .border_type(BorderType::Rounded)
                .style(Style::default())
                .title(titel)
                .title_alignment(Alignment::Center),
        );
    project
}
