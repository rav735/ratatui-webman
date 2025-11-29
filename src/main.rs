use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{error::Error, io};

// General
mod app;
mod ui;

//  UI - Elements
mod utils;

use crate::{
    app::{App, CurrentlyEditing},
    ui::{
        editor::{create_text_area, get_editor_style},
        ui::ui,
    },
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
    loop {
        editor_area.set_style(get_editor_style(app, &editor_area));
        editor_area.set_line_number_style(get_editor_style(app, &editor_area));

        terminal.draw(|f| ui(f, app, &editor_area))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            if app.currently_editing != CurrentlyEditing::RequestBody
                && key.code == event::KeyCode::Char('e')
            {
                app.currently_editing = CurrentlyEditing::RequestBody;
                continue;
            }

            if key.code == event::KeyCode::Esc {
                if app.currently_editing == CurrentlyEditing::RequestBody {
                    app.currently_editing = CurrentlyEditing::None;
                } else {
                    return Ok(false);
                }
            } else if app.currently_editing == CurrentlyEditing::RequestBody {
                editor_area.input(key);
            } else {
            }
        }
    }
}
