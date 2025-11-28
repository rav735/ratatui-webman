use crate::example_data::get_example_data;
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
}
#[derive(Debug)]
pub enum CurrentlyEditing {}

pub struct App {
    pub value: Map<String, Value>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
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
