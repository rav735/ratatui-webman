use std::collections::HashMap;

use itertools::Itertools;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Padding},
};

#[derive(PartialEq, Clone)]
pub struct DebugValue {
    pub category: String,
    pub name: String,
    pub value: String,
    pub style: Style,
    pub message: String,
}

impl DebugValue {
    pub fn update_style(&mut self, style: Style) {
        self.style = style
    }
}

pub struct Debugger {
    pub values: Vec<DebugValue>,
    pub style: Style,
}

impl Debugger {
    pub fn create_new() -> Debugger {
        Debugger {
            values: vec![],
            style: Style::default().fg(Color::LightCyan),
        }
    }

    pub fn add(&mut self, category: &String, name: &String, value: &String) {
        let hotkey: DebugValue = DebugValue {
            category: category.to_string(),
            name: name.to_string(),
            value: value.to_string(),
            style: (self.style),
            message: format!("[{}] - {}", name, value),
        };
        if self.exists(&hotkey) {
            let index = self
                .values
                .iter()
                .position(|v| v.category == category.to_string() && v.name == name.to_string())
                .unwrap();
            let mut old = self.values[index].clone();
            old.update_style(self.style);
            old.message = hotkey.message;
            self.values[index] = old;
        } else {
            self.values.push(hotkey);
        }
    }

    fn exists(&self, new_hk: &DebugValue) -> bool {
        self.values
            .iter()
            .any(|hk| hk.name == new_hk.name && hk.category == new_hk.category)
    }

    pub fn create_debugger_panel<'a>(&self, frame: &mut Frame, rec: Rect) {
        let mut dbg_lines: HashMap<String, Vec<String>> = HashMap::new();
        let mut list_list = vec![];

        self.add_debugging_lines(&mut dbg_lines);
        self.create_lists_by_categories(&mut dbg_lines, &mut list_list);

        let mut con: Vec<Constraint> = vec![Constraint::Length(1)];
        let length: i16 = dbg_lines.keys().len().try_into().unwrap();
        let mut sub_length: u16 = 0;
        dbg_lines.keys().sorted().for_each(|k| {
            dbg_lines.get(k).unwrap().iter().sorted().for_each(|message| {
                sub_length = sub_length + u16::try_from(message.lines().count()).ok().unwrap();
            });
            con.push(Constraint::Length(sub_length+1));
        });

        let list_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Color::Cyan)
            .style(Style::default())
            .title(Line::from(" Debugger ".to_string()).centered());

        frame.render_widget(list_block, rec);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(con)
            .split(rec);
        for i in 0..length as usize {
            frame.render_widget(list_list[i].clone(), layout[i + 1]);
        }
    }

    fn create_lists_by_categories(
        &self,
        dbg_lines: &mut HashMap<String, Vec<String>>,
        list_list: &mut Vec<List<'_>>,
    ) {
        for (category, messages) in dbg_lines.iter_mut().sorted() {
            messages.sort();
            let mut sub_list: Vec<ListItem> = vec![];
            for message in messages {
                sub_list.push(ListItem::new(Text::styled(message.clone(), self.style)));
            }
            let m: String = format!("{}", &category.clone());
            list_list.push(List::new(sub_list).block(get_debugger_block(m)))
        }
    }

    fn add_debugging_lines(&self, dbg_lines: &mut HashMap<String, Vec<String>>) {
        for debug_value in self.values.clone() {
            if dbg_lines.contains_key(&debug_value.category) {
                dbg_lines
                    .get_mut(&debug_value.category)
                    .unwrap()
                    .push(debug_value.message);
            } else {
                dbg_lines.insert(debug_value.category, vec![debug_value.message]);
            }
        }
    }
}

fn get_debugger_block(title: String) -> Block<'static> {
    return Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Plain)
        .border_style(Color::LightCyan)
        .style(Style::default())
        .title(Line::from(title).centered())
        .title_alignment(Alignment::Center)
        .padding(Padding { left: 1, right: 0, top: 0, bottom: 0 });
}