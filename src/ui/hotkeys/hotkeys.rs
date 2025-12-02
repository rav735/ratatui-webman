use chrono::{DateTime, TimeDelta, Utc};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};
use std::vec;

#[derive(PartialEq, Clone)]
pub struct Hotkey {
    pub category: String,
    pub name: String,
    pub hotkey: String,
    pub message: String,
    pub style: Style,
    pub style_frames_left: DateTime<Utc>,
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
        "\n".to_string() + &serde_json::to_string_pretty(&self.as_array()).unwrap()
    }
}

pub struct Hotkeys {
    pub values: Vec<Hotkey>,
    pub default_style: Style,
    pub clicked_style: Style,
}

impl Hotkeys {
    pub fn create_new() -> Hotkeys {
        let mut hotkeys = Hotkeys {
            values: vec![],
            default_style: Style::default().fg(Color::Gray),
            clicked_style: Style::default().fg(Color::DarkGray).bg(Color::Gray),
        };

        hotkeys.add(
            "s".to_string(),
            "Select Request".to_string(),
            "Change View".to_string(),
        );

        hotkeys.add(
            "e".to_string(),
            "Edit Request".to_string(),
            "Change View".to_string(),
        );
        hotkeys
    }

    // pub fn rm(&mut self, hotkey: String) {
    //     let c = match self.values.iter().position(|v| v.hotkey == hotkey) {
    //         Some(c) => c,
    //         None => 9999,
    //     };
    //     if c != 9999 {
    //         self.values.remove(c);
    //     }
    // }

    pub fn add(&mut self, hotkey: String, name: String, category: String) {
        let hotkey: Hotkey = Hotkey {
            category: category,
            name: name.clone(),
            hotkey: hotkey.clone(),
            message: format!("[{}] - {}", hotkey, name),
            style: self.default_style,
            style_frames_left: Utc::now(),
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
        let list_block = Block::default()
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
            .border_type(BorderType::Rounded)
            .border_style(Color::Gray)
            .style(Style::default())
            .title(Line::from(" Hotkeys ".to_string()).centered());

        let mut list_items = Vec::<ListItem>::new();

        for hk in self.values.clone() {
            list_items.push(ListItem::new(Line::from(Span::styled(
                hk.message, hk.style,
            ))));
        }

        self.values
            .iter_mut()
            .filter(|v| v.style_frames_left.timestamp_millis() > Utc::now().timestamp_millis())
            .for_each(|hk| hk.style = self.default_style);

        let list = List::new(list_items).block(list_block);
        list
    }

    pub fn check_for_hotkey_input(&mut self, key: &String) {
        self.values
            .iter()
            .position(|v| v.hotkey == key.to_string())
            .and_then(|index| {
                let mut old = self.values[index].clone();
                old.update_style(self.clicked_style);
                old.style_frames_left = Utc::now()
                    .checked_add_signed(TimeDelta::milliseconds(500))
                    .unwrap();
                self.values[index] = old;
                Some(0)
            });
    }
}
