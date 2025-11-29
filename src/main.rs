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
use tui_textarea::TextArea;

// General
mod app;
mod file;
mod ui;

//  UI - Elements
mod utils;

use crate::{
    app::{App, CurrentlyInteracting}, file::update_saved_request, ui::{
        editor::{create_text_area, get_editor_style},
        ui::ui,
    }
};

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
    let mut editor_area = create_text_area(app);
    let mut last_selected: String = app.saved_list.selected.clone();
    loop {
        editor_area.set_style(get_editor_style(app, &editor_area));
        editor_area.set_line_number_style(get_editor_style(app, &editor_area));

        terminal.draw(|f| ui(f, app, &editor_area))?;

        if last_selected != app.saved_list.selected.clone() {
            editor_area = create_text_area(app);
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            if app.currently_interacting != CurrentlyInteracting::SavedRequests
                && key.code == event::KeyCode::Char('1')
            {
                app.currently_interacting = CurrentlyInteracting::SavedRequests;
                app.saved_list.enable();
                continue;
            }

            if app.currently_interacting != CurrentlyInteracting::RequestBody
                && key.code == event::KeyCode::Char('e')
            {
                app.currently_interacting = CurrentlyInteracting::RequestBody;
                continue;
            }

            if key.code == event::KeyCode::Esc {
                if app.currently_interacting == CurrentlyInteracting::RequestBody {
                    app.currently_interacting = CurrentlyInteracting::SavedRequests;
                    continue;
                } else if app.currently_interacting == CurrentlyInteracting::SavedRequests {
                    app.currently_interacting = CurrentlyInteracting::None;
                    app.saved_list.disable();
                    continue;
                } else {
                    return Ok(false);
                }
            } else if app.currently_interacting == CurrentlyInteracting::SavedRequests {
                app.saved_list.handle_key(key);
            } else if app.currently_interacting == CurrentlyInteracting::RequestBody {
                if key.code == event::KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    update_saved_request(&last_selected, editor_area.clone().into_lines());
                }
                editor_area.input(key);
            } else {
            }
            last_selected = app.saved_list.selected.clone();
        }
    }
}
