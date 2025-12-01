use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding},
};
use tui_textarea::TextArea;

use crate::{EditorState, file::read_saved_request};

pub struct EditorTextArea<'a> {
    pub area: TextArea<'a>,
    style_enabled: Style,
    style_disabled: Style,
    style_line_number_enabled: Style,
    style_line_number_disabled: Style,
    style_cursor_body_enabled: Style,
    style_cursor_body_disabled: Style,
}

impl<'a> EditorTextArea<'a> {
    pub fn create_new(path: &String) -> EditorTextArea<'a> {
        let block = Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .border_type(BorderType::Rounded)
            .style(Style::default())
            .title(Line::from(" Request Body ").centered())
            .padding(Padding {
                left: 3,
                right: 0,
                top: 1,
                bottom: 1,
            });

        let mut a = TextArea::from(read_saved_request(path.to_string()).lines());
        a.set_block(block);
        a.set_alignment(Alignment::Left);

        let res = EditorTextArea {
            area: a,

            style_enabled: Style::default().fg(Color::Gray),
            style_line_number_enabled: Style::default().fg(Color::Gray),
            style_cursor_body_enabled: Style::default().fg(Color::LightBlue),

            style_disabled: Style::default().fg(Color::DarkGray),
            style_line_number_disabled: Style::default().fg(Color::DarkGray),
            style_cursor_body_disabled: Style::default().fg(Color::DarkGray),
        };
        res
    }

    pub fn update_text_style(&mut self, state: &EditorState) {
        if *state == EditorState::Editing {
            self.area.set_style(self.style_enabled);
            self.area
                .set_line_number_style(self.style_line_number_enabled);
            self.area
                .set_cursor_line_style(self.style_cursor_body_enabled);
        } else {
            self.area.set_style(self.style_disabled);
            self.area
                .set_line_number_style(self.style_line_number_disabled);
            self.area
                .set_cursor_line_style(self.style_cursor_body_disabled);
        }
    }

    pub fn get_current_content(&mut self) -> Vec<String> {
        return self.area.clone().into_lines();
    }
}
