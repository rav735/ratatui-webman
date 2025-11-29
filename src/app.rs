use crate::{ui::saved_requests::SavedList, utils::get_example_data};
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentlyEditing {
    None,
    RequestBody,
}

pub struct App {
    pub value: Map<String, Value>,
    pub current_screen: CurrentScreen,
    pub currently_editing: CurrentlyEditing,
    pub saved_list: SavedList<'static>
}

impl App {
    pub fn new() -> App {
        App {
            value: get_example_data(),
            current_screen: CurrentScreen::Main,
            currently_editing: CurrentlyEditing::None,
            saved_list: SavedList::default()
        }
    }

    pub fn update_saved_list(&mut self)
    {
        let shortcuts = vec![
            format!("app.current_screen : {:#?}", self.current_screen).to_string(),
            format!("app.currently_editing : {:#?}", self.currently_editing).to_string(),
        ];
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
