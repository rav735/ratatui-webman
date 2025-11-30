use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyModifiers},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{error::Error, io};

// General
mod app;
mod file;
mod ui;

//  UI - Elements
mod utils;

use crate::{
    app::{App, CurrentScreen},
    file::update_saved_request,
    ui::{editor::EditorTextArea, saved_requests::SavedRequestList, ui::ui}, utils::DebugValues,
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

    let mut debug_values = DebugValues::create_new();
    
    saved_list.update_list(&state);
    editor_area.update_text_style(&state);
    debug_values.add("State".to_string(), format!("{state:?}"));
    debug_values.add("Last".to_string(), last_selected.to_string());
    debug_values.add("Selected".to_string(), saved_list.selected.to_string());
    loop {

        
        terminal.draw(|f| ui(f, &saved_list.list, &editor_area.area, &debug_values))?;
        if last_selected != saved_list.selected{
            debug_values.add("Last".to_string(), last_selected.to_string());
            debug_values.add("Selected".to_string(), saved_list.selected.to_string());
            editor_area = EditorTextArea::create_new(&saved_list.selected);
            editor_area.update_text_style(&state);
            last_selected = saved_list.selected.clone();
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            if app.current_screen == CurrentScreen::Main {
                let mut state_change = false;
                if state == EditorState::Unfocused && key.code == event::KeyCode::Char('1') {
                    state = EditorState::SelectingRequest;
                    state_change = true;
                }

                if state == EditorState::SelectingRequest && key.code == event::KeyCode::Char('e') {
                    state = EditorState::Editing;
                    state_change = true;
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
                    debug_values.add("State".to_string(), format!("{state:?}"));
                    continue;
                }

                if state == EditorState::SelectingRequest {
                    last_selected = saved_list.selected.clone();
                    saved_list.handle_key(key);
                    saved_list.update_list(&state);
                } else if state == EditorState::Editing {
                    editor_handle_input(&mut editor_area, &last_selected, key);
                } else {
                }
            }
        }
    }
}

fn editor_handle_input(editor_area: &mut EditorTextArea<'_>, last_selected: &String, key: event::KeyEvent) {
    if key.code == event::KeyCode::Char('s')
        && key.modifiers.contains(KeyModifiers::CONTROL)
    {
        update_saved_request(last_selected, editor_area.get_current_content());
    }
    else {
        editor_area.area.input(key);
    }
}