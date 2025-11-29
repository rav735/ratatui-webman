use std::iter::Map;

use crate::{app::App, utils::create_list};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind}, style::{Color, Style}, widgets::{Borders, List}
};

#[derive(Default)]
pub struct SavedList<'a> {
    pub values : Vec<String>,
    styles : Vec<Style>,
    selected : i32,
    pub ui_element : List<'a>
}

impl SavedList<'_>{
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
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

    fn select_none(&mut self) {
        self.selected = -1;
    }

    fn select_next(&mut self) {
        self.selected += 1;
    }
    fn select_previous(&mut self) {
        self.selected -= 1;
    }

    fn select_first(&mut self) {
        self.selected = 0;
    }

    fn select_last(&mut self) {
        self.selected = self.values.len() as i32;
    }

    pub fn create_saved_list<'a>(&mut self) -> List<'a>
    {
        let list = create_list(
            " [1] - Saved Requests ".to_string(),
            self.values.clone(),
            Color::DarkGray,
            Color::Gray,
            Borders::ALL
        );
        list
    }
}
