use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Padding},
};
use tui_textarea::TextArea;

use crate::app::App;

pub fn create_text_area<'a>(app: &App) -> TextArea<'a> {
    let style = Style::default().bg(Color::DarkGray);
    let mut textarea = TextArea::from(serde_json::to_string_pretty(&app.value).unwrap().lines());
    textarea.set_line_number_style(style);
    textarea.set_block(
        Block::default()
            .borders(Borders::TOP)
            .style(Style::default())
            .title(Line::from(" Request Body ").centered())
            .padding(Padding {
                left: 3,
                right: 0,
                top: 1,
                bottom: 1,
            }),
    );
    textarea
}
