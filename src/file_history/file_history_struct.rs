use std::{collections::HashMap, usize};
use chrono::Utc;

const SAVED_REQUEST_PATH: &str = "./saved_requests/";

pub struct FileHistory {
    pub name: String,
    pub path: String,
    pub changes: HashMap<usize, (String, String)>,
    pub timestmap: String,
    pub version_number: usize,
    pub saved: bool,
}

impl FileHistory {
    pub fn create_new(path: String) -> FileHistory {
        FileHistory {
            name: path.replace(SAVED_REQUEST_PATH, ""),
            path: path,
            changes: HashMap::new(),
            timestmap: Utc::now()
                .to_string()
                .split('.')
                .into_iter()
                .nth(0)
                .unwrap()
                .to_owned(),
            version_number: 0,
            saved: false,
        }
    }
    
    pub fn set_old_row(&mut self, row: usize, old: String) {
        self.changes.insert(row, (old, "".to_string()));
    }
    pub fn set_new_row(&mut self, row: usize, new: String) {
        self.changes.get_mut(&row).and_then(|f| Some(f.1 = new));
    }

    pub fn as_array(&self) -> [String; 5] {
        [
            self.name.clone(),
            self.path.clone(),
            self.timestmap.clone(),
            self.version_number.to_string(),
            self.saved.to_string(),
        ]
    }

    pub fn to_string_pretty(&mut self) -> String {
        "\n".to_string() + &serde_json::to_string_pretty(&self.as_array()).unwrap()
    }
}
