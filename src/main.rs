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
    model::app::{state::DisplayMode, App},
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
    app.set_add_table_mode()?;
    app.add_table("Table 1")?;
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
        terminal.draw(|f| ui(f, app))?;
        // handle key event
        if let Event::Key(key) = event::read()? {
            let message = handle_key_event(key, app);
            if let Message::Exit = message {
                break;
            }
            update(app, message)?;
        }
    }

    Ok(())
}

fn handle_key_event(key: KeyEvent, app: &App) -> Message {
    match key.code {
        KeyCode::Char(c)
            if (*app.get_display_mode() == DisplayMode::EditCell
                || *app.get_display_mode() == DisplayMode::AddTable
                || *app.get_display_mode() == DisplayMode::EditHeader) =>
        {
            Message::Input(c)
        }
        KeyCode::Char('q') => match app.get_exit() {
            true => Message::Exit,
            false => Message::Exiting,
        },
        KeyCode::Esc => match app.get_display_mode() {
            DisplayMode::AddTable => Message::SelectTableMode,
            DisplayMode::SelectCell => Message::SelectTableMode,
            DisplayMode::EditCell => Message::SelectCellMode,
            _ => Message::NoOp,
        },
        KeyCode::Enter => match app.get_display_mode() {
            DisplayMode::AddTable => Message::NewTable(app.get_user_input().to_string()),
            DisplayMode::SelectTable => Message::SelectTable,
            DisplayMode::SelectCell => Message::EditCellMode,
            DisplayMode::EditHeader => Message::SaveHeader(app.get_user_input().to_string()),
            DisplayMode::EditCell => Message::SaveCellValue(app.get_user_input().to_string()),
        },
        KeyCode::Char('a') => match app.get_display_mode() {
            DisplayMode::SelectTable => Message::AddTableMode,
            _ => Message::NoOp,
        },
        KeyCode::Char('d') => match app.get_display_mode() {
            DisplayMode::SelectTable => Message::RemoveTable,
            DisplayMode::SelectCell => Message::SaveCellValue("".to_string()),
            _ => unreachable!(),
        },
        KeyCode::Char('e') => match app.get_display_mode() {
            DisplayMode::SelectCell => Message::EditCellMode,
            _ => Message::NoOp,
        },
        KeyCode::Char('H') => match app.get_display_mode() {
            DisplayMode::SelectCell => Message::EditHeaderMode,
            _ => Message::NoOp,
        },
        KeyCode::Char('r') => match app.get_display_mode() {
            DisplayMode::SelectCell => Message::ExpandRow,
            _ => Message::NoOp,
        },
        KeyCode::Char('R') => match app.get_display_mode() {
            DisplayMode::SelectCell => Message::CollapseRow,
            _ => Message::NoOp,
        },
        KeyCode::Char('c') => match app.get_display_mode() {
            DisplayMode::SelectCell => Message::ExpandColumn,
            _ => Message::NoOp,
        },
        KeyCode::Char('C') => match app.get_display_mode() {
            DisplayMode::SelectCell => Message::CollapseColumn,
            _ => Message::NoOp,
        },
        // move cursor
        KeyCode::Up => Message::Move(MoveDirection::Up),
        KeyCode::Down => Message::Move(MoveDirection::Down),
        KeyCode::Left => Message::Move(MoveDirection::Left),
        KeyCode::Right => Message::Move(MoveDirection::Right),
        KeyCode::Tab if *app.get_display_mode() == DisplayMode::SelectCell => {
            Message::Move(MoveDirection::Right)
        }

        // vim keybindings
        KeyCode::Char('h') if *app.get_display_mode() == DisplayMode::SelectCell => {
            Message::Move(MoveDirection::Left)
        }
        KeyCode::Char('j')
            if *app.get_display_mode() == DisplayMode::SelectCell
                || *app.get_display_mode() == DisplayMode::SelectTable =>
        {
            Message::Move(MoveDirection::Down)
        }
        KeyCode::Char('k')
            if *app.get_display_mode() == DisplayMode::SelectCell
                || *app.get_display_mode() == DisplayMode::SelectTable =>
        {
            Message::Move(MoveDirection::Up)
        }
        KeyCode::Char('l') if *app.get_display_mode() == DisplayMode::SelectCell => {
            Message::Move(MoveDirection::Right)
        }

        KeyCode::Backspace => {
            // app.pop_user_input();
            // continue;
            Message::PopInput
        }

        _ => Message::NoOp,
    }
}
