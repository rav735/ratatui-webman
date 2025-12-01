use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::{EditorState, file::get_saved_requests};

#[derive(Default)]
pub struct SavedRequestList<'a> {
    pub values: Vec<String>,
    pub list: List<'a>,
    pub list_block: Block<'a>,

    pub selected: String,
    pub selected_index: usize,

    _selected_style: Style,
    default_style: Style,
    _disabled_style: Style,
}

impl<'a> SavedRequestList<'a> {
    pub fn create_new() -> SavedRequestList<'a> {
        let list_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Color::Gray)
            .border_type(BorderType::Rounded)
            .style(Style::default())
            .title(Line::from(" [1] - Saved Requests ").centered());

        let list = List::default();

        let res = SavedRequestList {
            values: get_saved_requests(),
            list,
            list_block,
            selected: get_saved_requests()[0].clone(),
            selected_index: 0,
            _selected_style: Style::new().bg(Color::Gray).fg(Color::Gray),
            default_style: Style::new().fg(Color::Gray),
            _disabled_style: Style::new().fg(Color::DarkGray),
        };
        res
    }

    pub fn update_list(&mut self, state: &EditorState) {
        let mut prefixes: Vec<String> = vec![];
        self.get_styles_based_on_state(&mut prefixes, *state);

        let mut new_values: Vec<ListItem> = vec![];

        for i in 0..self.values.len() {
            new_values.push(ListItem::new(Line::from(Span::styled(
                prefixes[i].clone() + &self.values[i].to_string(), self.default_style
            ))));
        }

        self.list = List::new(new_values).block(self.list_block.clone());
    }

    fn get_styles_based_on_state(&mut self, prefixes: &mut Vec<String>, state: EditorState) {
        for sr in self.values.clone() {
            if sr == self.values.get(self.selected_index).unwrap().clone() {
                prefixes.push(">> ".to_string());
                self.selected = sr;
            }

            if state == EditorState::SelectingRequest {
                prefixes.push("".to_string());
            } else {
                prefixes.push("-".to_string());
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            // KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('k') | KeyCode::Down => self.select_next(),
            KeyCode::Char('j') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            // KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            //     self.toggle_status();
            // }
            _ => {}
        }
    }

    fn select_next(&mut self) {
        if self.selected_index == self.values.len() - 1 {
            return;
        }
        self.selected_index += 1;
    }
    fn select_previous(&mut self) {
        if self.selected_index == 0 {
            return;
        }
        self.selected_index -= 1;
    }
    fn select_first(&mut self) {
        self.selected_index = 0;
    }
    fn select_last(&mut self) {
        self.selected_index = self.values.len() - 1;
    }
}
