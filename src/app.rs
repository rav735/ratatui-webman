use crate::{
    file::get_saved_requests, ui::saved_requests::SavedRequestList, utils::get_example_data,
};
use ratatui::{
    style::{Color, Style},
    widgets::List,
};
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentlyInteracting {
    None,
    SavedRequests,
    RequestBody,
}

pub struct App {
    pub value: Map<String, Value>,
    pub current_screen: CurrentScreen,
    pub currently_interacting: CurrentlyInteracting,
    pub saved_list: SavedRequestList<'static>,
}

impl App {
    pub fn new() -> App {
        App {
            value: get_example_data(),
            current_screen: CurrentScreen::Main,
            currently_interacting: CurrentlyInteracting::None,
            saved_list: SavedRequestList {
                values: get_saved_requests(),
                ui_element: List::default(),
                selected: get_saved_requests()[0].clone(),
                selected_index: 0,
                disabled: true,
                selected_style: Style::new().bg(Color::Gray).fg(Color::Gray),
                default_style: Style::new().fg(Color::Gray),
                disabled_style: Style::new().fg(Color::DarkGray),
            },
        }
    }

    pub fn update_saved_list(&mut self) {
        let shortcuts = get_saved_requests();
        self.saved_list.values = shortcuts;
        let temp = self.saved_list.create_saved_list();
        self.saved_list.ui_element = temp;
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string_pretty(&self.value)?;
        println!("---------------");
        println!("{output}");
        println!("---------------");
        Ok(())
    }
}
