use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Padding},
};
use tui_textarea::TextArea;

use crate::{
    app::{App, CurrentlyInteracting},
    file::read_saved_request,
};

pub fn create_text_area<'a>(app: &App) -> TextArea<'a> {
    let style = Style::default().bg(Color::DarkGray);
    let mut textarea = TextArea::from(read_saved_request(app.saved_list.selected.clone()).lines());
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

pub fn get_editor_style<'a>(app: &App, area: &TextArea<'a>) -> Style {
    let mut style = area.style();
    if app.currently_interacting == CurrentlyInteracting::RequestBody {
        style = style.fg(Color::Gray);
    } else {
        style = style.fg(Color::DarkGray);
    }
    style
}
