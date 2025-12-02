use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::ui::saved_requests::SavedRequestList;

impl<'a> SavedRequestList<'a> {
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
