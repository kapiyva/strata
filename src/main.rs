use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use strata::{
    app::{display_focus::DisplayFocus, App},
    message::{Message, MoveDirection},
    update::update,
    view::view,
};

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // run app
    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    // cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        if let Err(e) = terminal.draw(|f| view(f, app)) {
            app.error_popup_mut().push(e.to_string());
            app.focus_error();
        }

        if let Event::Key(key) = event::read()? {
            let message = handle_key_event(key, app.display_focus());
            if let Message::Exit = message {
                break;
            }
            if let Err(e) = update(app, message) {
                app.error_popup_mut().push(e.to_string());
                app.focus_error();
            }
        }
    }

    Ok(())
}

fn handle_key_event(key: KeyEvent, focus: &DisplayFocus) -> Message {
    match key.code {
        // input charactor
        KeyCode::Char(c) if matches!(focus, DisplayFocus::Command(_)) => Message::Input(c),
        // special key
        KeyCode::Esc => Message::Cancel,
        KeyCode::Enter => match focus {
            DisplayFocus::Command(_) => Message::ExecuteCommand,
            DisplayFocus::TableSelector => Message::SelectTable,
            DisplayFocus::TableView => Message::EditCell,
            DisplayFocus::Exit(_) => Message::Exit,
            _ => Message::NoOp,
        },
        KeyCode::Backspace => Message::PopInput,
        // others
        KeyCode::Char('q') => match focus {
            DisplayFocus::Exit(_) => {
                return Message::Exit;
            }
            DisplayFocus::TableSelector => Message::Exiting,
            DisplayFocus::TableView => Message::Cancel,
            _ => Message::NoOp,
        },
        KeyCode::Char('a') => match focus {
            DisplayFocus::TableSelector => Message::AddTable,
            DisplayFocus::TableView => Message::EditCell,
            _ => Message::NoOp,
        },
        KeyCode::Char('d') => match focus {
            DisplayFocus::TableSelector => Message::RemoveTable,
            DisplayFocus::TableView => Message::DeleteCell,
            _ => Message::NoOp,
        },
        KeyCode::Char('e') => match focus {
            DisplayFocus::TableSelector => Message::EditTableName,
            DisplayFocus::TableView => Message::EditCell,
            _ => Message::NoOp,
        },
        KeyCode::Char('E') => match focus {
            DisplayFocus::TableView => Message::EditHeader,
            _ => Message::NoOp,
        },
        KeyCode::Char('o') => match focus {
            DisplayFocus::TableSelector => Message::Open,
            _ => Message::NoOp,
        },
        KeyCode::Char('s') => match focus {
            DisplayFocus::TableSelector | DisplayFocus::TableView => Message::Save,
            _ => Message::NoOp,
        },
        KeyCode::Char('r') => match focus {
            DisplayFocus::TableView => Message::ExpandRow,
            _ => Message::NoOp,
        },
        KeyCode::Char('R') => match focus {
            DisplayFocus::TableView => Message::CollapseRow,
            _ => Message::NoOp,
        },
        KeyCode::Char('c') => match focus {
            DisplayFocus::TableView => Message::ExpandColumn,
            _ => Message::NoOp,
        },
        KeyCode::Char('C') => match focus {
            DisplayFocus::TableView => Message::CollapseColumn,
            _ => Message::NoOp,
        },
        // move cursor
        KeyCode::Up => Message::Move(MoveDirection::Up),
        KeyCode::Down => Message::Move(MoveDirection::Down),
        KeyCode::Right => Message::Move(MoveDirection::Right),
        KeyCode::Left => Message::Move(MoveDirection::Left),
        KeyCode::Tab if *focus == DisplayFocus::TableView => Message::Move(MoveDirection::Right),
        KeyCode::Char('J') => match focus {
            DisplayFocus::TableView => Message::JumpTable,
            DisplayFocus::TableSelector => Message::JumpCell,
            _ => Message::NoOp,
        },

        // vim keybindings
        KeyCode::Char('h') if *focus == DisplayFocus::TableView => {
            Message::Move(MoveDirection::Left)
        }
        KeyCode::Char('j')
            if *focus == DisplayFocus::TableView || *focus == DisplayFocus::TableSelector =>
        {
            Message::Move(MoveDirection::Down)
        }
        KeyCode::Char('k')
            if *focus == DisplayFocus::TableView || *focus == DisplayFocus::TableSelector =>
        {
            Message::Move(MoveDirection::Up)
        }
        KeyCode::Char('l')
            if *focus == DisplayFocus::TableView || *focus == DisplayFocus::TableSelector =>
        {
            Message::Move(MoveDirection::Right)
        }
        _ => Message::NoOp,
    }
}
