use std::{collections::HashMap, iter::Map};

use ratatui::{style::Color, widgets::{Borders, List}};

use crate::utils::create_list;

pub struct DebugValues{
    pub values: HashMap<String,String>,
}

impl DebugValues {
    pub fn create_new() -> DebugValues
    {
        DebugValues{ values: HashMap::new() }
    }

    pub fn add(&mut self, name: String, val: String)
    {
        if self.values.contains_key(&name)
        {
            let k = self.values.get_mut(&name).unwrap();
            *k = val;
        }
        else {
            self.values.insert(name,val);
        }
    }

    pub fn create_debuger_panel<'a>(&self) -> List<'a> {
        let mut debugging_lines = vec![];
        for (k,v) in &self.values{
            debugging_lines.push(format!("{k} - {v}"));
        }

// State state
// Last last_sel
// Selected sel

        let list = create_list(
            " Debugger ".to_string(),
            debugging_lines,
            Color::LightYellow,
            Color::LightCyan,
            Borders::ALL,
        );
        list
    }
}