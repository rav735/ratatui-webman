use crate::
    utils::get_example_data
;
use serde_json::{Map, Value};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CurrentScreen {
    Main,
    // History,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub enum CurrentlyInteracting {
//     None,
//     SavedRequests,
//     RequestBody,
// }

pub struct App {
    pub value: Map<String, Value>,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            value: get_example_data(),
            current_screen: CurrentScreen::Main,
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
