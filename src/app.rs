use serde_json::{Map, Value};
use tui_textarea::TextArea;
use crate::example_data::get_example_data;

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}
#[derive(Debug)]
pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub value: Map<String, Value>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App{
    pub fn new() -> App{
        App {
            value: get_example_data(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string_pretty(&self.value)?;
        println!("---------------");
        println!("{output}");
        println!("---------------");
        Ok(())
    }
}
