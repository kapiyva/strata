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
    message::{Message, MoveDirection},
    model::app::{state::DisplayFocus, App},
    ui::ui,
    update::update,
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

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
    terminal.clear()?;
    loop {
        if let Err(e) = terminal.draw(|f| ui(f, app)) {
            app.push_error_message(e.to_string());
            app.focus_error();
        }
        // handle key event
        if let Event::Key(key) = event::read()? {
            let message = handle_key_event(key, app.get_display_focus());
            if let Message::Exit = message {
                break;
            }
            if let Err(e) = update(app, message) {
                app.push_error_message(e.to_string());
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
        KeyCode::Enter => {
            if let DisplayFocus::Exit(_) = focus {
                return Message::Exit;
            }
            Message::Enter
        }
        KeyCode::Backspace => Message::BackSpace,
        // others
        KeyCode::Char('q') => {
            if let DisplayFocus::Exit(_) = focus {
                return Message::Exit;
            }
            Message::Exiting
        }
        KeyCode::Char('a') => Message::Add,
        KeyCode::Char('d') => Message::Delete,
        KeyCode::Char('e') => Message::Edit,
        KeyCode::Char('E') => Message::HyperEdit,
        KeyCode::Char('o') => Message::Open,
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
        KeyCode::Char('J') => Message::Jump,

        // vim keybindings
        KeyCode::Char('h') if *focus == DisplayFocus::TableView => {
            Message::Move(MoveDirection::Left)
        }
        KeyCode::Char('j')
            if *focus == DisplayFocus::TableView || *focus == DisplayFocus::TableList =>
        {
            Message::Move(MoveDirection::Down)
        }
        KeyCode::Char('k')
            if *focus == DisplayFocus::TableView || *focus == DisplayFocus::TableList =>
        {
            Message::Move(MoveDirection::Up)
        }
        KeyCode::Char('l')
            if *focus == DisplayFocus::TableView || *focus == DisplayFocus::TableList =>
        {
            Message::Move(MoveDirection::Right)
        }
        _ => Message::NoOp,
    }
}
