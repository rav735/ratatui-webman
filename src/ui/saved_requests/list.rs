use crate::utils::create_list_with_styles;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind},
    style::{Color, Style},
    widgets::{Borders, List, Paragraph},
};

#[derive(Default)]
pub struct SavedRequestList<'a> {
    pub values: Vec<String>,
    pub ui_element: List<'a>,
    pub disabled: bool,

    pub selected: String,
    pub selected_index: usize,

    pub selected_style: Style,
    pub default_style: Style,
    pub disabled_style: Style,
}

impl SavedRequestList<'_> {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            // KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            // KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            //     self.toggle_status();
            // }
            _ => {}
        }
    }

    pub fn enable(&mut self) {
        self.disabled = false;
    }

    pub fn disable(&mut self) {
        self.disabled = true;
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

    pub fn create_saved_list<'a>(&mut self) -> List<'a> {
        let mut styles: Vec<Style> = vec![];
        for sr in self.values.clone() {
            if self.disabled {
                styles.push(self.disabled_style);
                continue;
            }
            if sr == self.values.get(self.selected_index).unwrap().clone() {
                styles.push(self.selected_style);
                self.selected = sr;
            } else {
                styles.push(self.default_style);
            }
        }

        let list = create_list_with_styles(
            " [1] - Saved Requests ".to_string(),
            self.values.clone(),
            styles,
            Color::Gray,
            Borders::ALL,
        );
        list
    }
}
