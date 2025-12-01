use ratatui::{
    crossterm::event::KeyCode,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};
use std::
    collections::HashMap
;

#[derive(PartialEq, Clone)]
pub struct Hotkey {
    pub category: String,
    pub name: String,
    pub hotkey: String,
    pub message: String,
    pub key_code: KeyCode,
    pub style: Style,
}

impl Hotkey {
    pub fn update_style(&mut self, style: Style) {
        self.style = style
    }

    pub fn as_array(&self) -> [String; 4] {
        [
            self.category.clone(),
            self.name.clone(),
            self.hotkey.clone(),
            self.message.clone(),
        ]
    }

    pub fn to_string_pretty(&mut self) -> String {
        serde_json::to_string_pretty(&self.as_array()).unwrap()
    }
}

pub struct Hotkeys {
    pub values: Vec<Hotkey>,
    pub default_style: Style,
    pub clicked_style: Style,
}

impl Hotkeys {
    pub fn create_new() -> Hotkeys {
        let mut hotkeys =Hotkeys {
            values: vec![],
            default_style: Style::default().fg(Color::Gray),
            clicked_style: Style::default().fg(Color::DarkGray).bg(Color::Gray),
        };

        hotkeys.add(
            "1".to_string(),
            "Select Request".to_string(),
            "Hotkeys".to_string(),
        );

        hotkeys.add(
            "e".to_string(),
            "Edit Request".to_string(),
            "Hotkeys".to_string(),
        );
        hotkeys
    }

    pub fn add(&mut self, hotkey: String, name: String, category: String) {
        let hotkey: Hotkey = Hotkey {
            category: category,
            name: name.clone(),
            hotkey: hotkey.clone(),
            message: format!("[{}] - {}", hotkey, name),
            key_code: KeyCode::Char(hotkey.chars().nth(0).unwrap()),
            style: self.default_style,
        };
        if self.exists(&hotkey) {
        } else {
            self.values.push(hotkey);
        }
    }

    fn exists(&self, new_hk: &Hotkey) -> bool {
        self.values.iter().any(|hk| {
            hk.name == new_hk.name && hk.category == new_hk.category && hk.hotkey == new_hk.hotkey
        })
    }

    pub fn create_hotkeys_panel<'a>(&mut self) -> List<'a> {
        let mut hk_lines: HashMap<Style, Vec<String>> = HashMap::new();
        hk_lines.insert(self.clicked_style, vec![]);
        hk_lines.insert(self.default_style, vec![]);

        for hk in self.values.clone() {
            hk_lines.get_mut(&hk.style).unwrap().push(hk.message);
        }

        let list_block = Block::default()
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
            .border_type(BorderType::Rounded)
            .border_style(Color::Gray)
            .style(Style::default())
            .title(Line::from(" Hotkeys ".to_string()).centered());

        let mut list_items = Vec::<ListItem>::new();

        for (style, messages) in hk_lines.iter_mut() {
            messages.sort();
            for message in messages {
                list_items.push(ListItem::new(Line::from(Span::styled(
                    message.to_string(),
                    *style,
                ))));
            }
        }

        self.values
            .iter_mut()
            .for_each(|hk| hk.style = self.default_style);

        let list = List::new(list_items).block(list_block);
        list
    }

    pub fn check_for_hotkey_input(&mut self, key: KeyCode) {
        self.values
            .iter()
            .position(|v| v.key_code == key)
            .and_then(|index| {
                let mut old = self.values[index].clone();
                old.update_style(self.clicked_style);
                self.values[index] = old;
                Some(0)
            });
    }
}
