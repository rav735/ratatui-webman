//? fn save_edits -> saves edit per line (see debugger)
//? fn read_edits -> gets edits (old + new) for a file, since last save
//? fn get_diff -> gets saved file content, replaces edited lines with old+new line (green/red - +/-)

use crate::file_history::file_history_struct::FileHistory;

impl FileHistory {
    pub fn save_edits(&self) {}
}
