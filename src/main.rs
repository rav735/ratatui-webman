use chrono::Utc;
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{error::Error, io};
use tui_textarea::CursorMove;

// General
mod app;
mod file;
mod file_history;
mod ui;

//  UI - Elements
mod utils;

use crate::{
    app::{App, CurrentScreen},
    file::update_saved_request,
    file_history::FileHistory,
    ui::{editor::EditorTextArea, hotkeys::Hotkeys, saved_requests::SavedRequestList, ui::ui},
    utils::Debugger,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EditorState {
    Unfocused,
    Editing,
    SelectingRequest,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    let mut state: EditorState = EditorState::SelectingRequest;

    let mut saved_list = SavedRequestList::create_new();
    saved_list.update_list(&state);

    let mut editor_area = EditorTextArea::create_new(&saved_list.selected);

    let mut last_selected = saved_list.selected.clone();

    let mut debugger = Debugger::create_new();
    let mut file_tracker = FileHistory::create_new(saved_list.selected.to_string());

    saved_list.update_list(&state);
    editor_area.update_text_style(&state);
    debugger.add(
        &"Runtime".to_string(),
        &"state".to_string(),
        &format!("{state:?}"),
        false,
    );
    debugger.add(
        &"Editor".to_string(),
        &"last_selected".to_string(),
        &last_selected.to_string(),
        false,
    );
    debugger.add(
        &"Editor".to_string(),
        &"saved_list.selected".to_string(),
        &saved_list.selected.to_string(),
        false,
    );
    let mut last_pos: (usize, usize) = (0, 0);

    let mut last_enabled_category: String = "".to_string();
    let mut last_input: String = "".to_string();
    let hotkeys: &mut Hotkeys = &mut Hotkeys::create_new();

    hotkeys.add(
        "F12".to_string(),
        "Toggle Debugger".to_string(),
        "Debugger".to_string(),
    );
    loop {
        debugger.add(
            &"Editing".to_string(),
            &"FileTracker".to_string(),
            &file_tracker.to_string_pretty(),
            true,
        );
        terminal.draw(|f| ui(f, &saved_list, &editor_area, hotkeys, &debugger))?;
        let dbg_panel_count = hotkeys
            .values
            .iter()
            .filter(|v| v.name.contains("Debug:"))
            .count();
        debugger.add(
            &"Runtime".to_string(),
            &"Dbg Category Count".to_string(),
            &dbg_panel_count.to_string(),
            false,
        );
        for mut hk in hotkeys.values.clone() {
            debugger.add(
                &"Hotkeys".to_string(),
                &hk.name.clone(),
                &hk.to_string_pretty(),
                true,
            );
        }
        debugger.add(
            &"Runtime".to_string(),
            &"state".to_string(),
            &format!("{:?}", &state.clone()),
            false,
        );
        if last_selected != saved_list.selected {
            debugger.add(
                &"Editor".to_string(),
                &"last_selected".to_string(),
                &last_selected.to_string(),
                false,
            );
            debugger.add(
                &"Editor".to_string(),
                &"saved_list.selected".to_string(),
                &saved_list.selected.to_string(),
                false,
            );
            editor_area = EditorTextArea::create_new(&saved_list.selected);
            editor_area.update_text_style(&state);
            last_selected = saved_list.selected.clone();
            file_tracker = FileHistory::create_new(saved_list.selected.to_string());
        }
        debugger.add(
            &"Runtime".to_string(),
            &"Last Enabled Category".to_string(),
            &last_enabled_category.to_string(),
            false,
        );
        debugger.add(
            &"Runtime".to_string(),
            &"Last Input".to_string(),
            &last_input.to_string(),
            false,
        );

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            last_input = key.code.to_string();
            hotkeys.check_for_hotkey_input(&last_input);

            if last_input.contains("F")
                && last_input.len() <= 3
                && (last_input
                    .replace("F", "")
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize
                    <= dbg_panel_count
                    || last_input == "F12")
            {
                if last_input == "F12" {
                    debugger.panel_enabled = !debugger.panel_enabled
                } else {
                    let cat = hotkeys
                        .values
                        .iter()
                        .filter(|hk| hk.hotkey == key.code.to_string())
                        .nth(0)
                        .unwrap()
                        .name
                        .replace("Debug: ", "")
                        .clone();
                    debugger.enable_category(cat.clone());
                    last_enabled_category = cat;
                }
            }

            if app.current_screen == CurrentScreen::Main {
                let mut state_change = false;
                if state == EditorState::Unfocused && key.code == event::KeyCode::Char('s') {
                    state = EditorState::SelectingRequest;
                    state_change = true;
                    debugger.add(
                        &"Runtime".to_string(),
                        &"Last Category Shortcut".to_string(),
                        &key.code.to_string(),
                        false,
                    );
                }

                if state == EditorState::SelectingRequest && key.code == event::KeyCode::Char('e') {
                    state = EditorState::Editing;
                    state_change = true;
                    debugger.add(
                        &"Runtime".to_string(),
                        &"Last Category Shortcut".to_string(),
                        &key.code.to_string(),
                        false,
                    );
                }

                if key.code == event::KeyCode::Esc {
                    if state == EditorState::Editing {
                        state = EditorState::SelectingRequest;
                        state_change = true;
                    } else if state == EditorState::SelectingRequest {
                        state = EditorState::Unfocused;
                        state_change = true;
                    } else {
                        return Ok(false);
                    }
                }

                if state_change {
                    editor_area.update_text_style(&state);
                    saved_list.update_list(&state);
                    continue;
                }

                if state == EditorState::SelectingRequest {
                    last_selected = saved_list.selected.clone();
                    saved_list.handle_key(key);
                    saved_list.update_list(&state);
                } else if state == EditorState::Editing {
                    last_pos = editor_handle_input(
                        &mut editor_area,
                        &last_selected,
                        key,
                        &mut debugger,
                        last_pos,
                        &mut file_tracker,
                    );
                    debugger.add(
                        &"Editing".to_string(),
                        &"Last Cursor Position".to_string(),
                        &format!("{:?}", last_pos),
                        false,
                    );
                } else {
                }
            }
        }
    }
}

fn editor_handle_input(
    editor_area: &mut EditorTextArea<'_>,
    last_selected: &String,
    key: event::KeyEvent,
    debugger: &mut Debugger,
    last_position: (usize, usize),
    file_tracker: &mut FileHistory,
) -> (usize, usize) {
    if key.code == event::KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
        update_saved_request(last_selected, editor_area.get_current_content());
        let dt = Utc::now();
        debugger.add(
            &"Editing".to_string(),
            &"Last saved at".to_string(),
            &format!(
                "{:?}",
                dt.to_string().split('.').into_iter().nth(0).unwrap()
            ),
            false,
        );
        file_tracker.saved = true;
        return last_position;
    } else {
        let cursor: (usize, usize) = editor_area.area.cursor();

        if last_position.0 != cursor.0 {
            //? Filter move keys?
            let yanked = yank_current_line(editor_area, cursor, false);
            file_tracker.set_old_row(cursor.0, yanked);
            debugger.add(
                &"Editing".to_string(),
                &"FileTracker".to_string(),
                &file_tracker.to_string_pretty(),
                true,
            );
        }
        if editor_area.area.input(key) {
            let new_line = key.code == KeyCode::Enter;
            let cursor: (usize, usize) = editor_area.area.cursor();
            let yanked = yank_current_line(editor_area, cursor, new_line);
            file_tracker.set_new_row(if new_line { cursor.0 - 1 } else { cursor.0 }, yanked);
            debugger.add(
                &"Editing".to_string(),
                &"FileTracker".to_string(),
                &file_tracker.to_string_pretty(),
                true,
            );
        }
        cursor
    }
}

fn yank_current_line(
    editor_area: &mut EditorTextArea<'_>,
    cursor: (usize, usize),
    new_line: bool,
) -> String {
    if new_line {
        editor_area
            .area
            .move_cursor(CursorMove::Jump((cursor.0 - 1) as u16, 0 as u16));
    }
    editor_area.area.move_cursor(CursorMove::Head);
    editor_area.area.start_selection();
    editor_area.area.move_cursor(CursorMove::End);
    editor_area.area.copy();
    editor_area
        .area
        .move_cursor(CursorMove::Jump(cursor.0 as u16, cursor.1 as u16));
    let res = editor_area.area.yank_text();
    editor_area.area.cancel_selection();
    res
}
