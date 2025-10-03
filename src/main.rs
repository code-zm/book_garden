mod app;
mod storage;
mod ui;
mod plant;
mod book_select;

use std::{
    io::stdout,
    time::Duration,
};

use app::{AppState, TimerState};
use book_select::{BookSelector, InputMode, draw_book_select};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use storage::{load_library, add_book};

enum AppMode {
    BookSelection,
    Reading,
}

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut library = load_library()?;
    let mut selector = BookSelector::new();
    let mut app_mode = AppMode::BookSelection;
    let mut app: Option<AppState> = None;

    loop {
        match app_mode {
            AppMode::BookSelection => {
                terminal.draw(|f| draw_book_select(f, &library, &selector))?;

                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                        match selector.input_mode {
                            InputMode::Selection => match code {
                                KeyCode::Up => selector.select_prev(&library),
                                KeyCode::Down => selector.select_next(&library),
                                KeyCode::Enter => {
                                    if !library.books.is_empty() && selector.selected_index < library.books.len() {
                                        let book = &library.books[selector.selected_index];
                                        app = Some(AppState::new(selector.selected_index, book));
                                        app_mode = AppMode::Reading;
                                    }
                                }
                                KeyCode::Char('n') | KeyCode::Char('N') => {
                                    selector.input_mode = InputMode::EnteringTitle;
                                    selector.new_book_title.clear();
                                    selector.new_book_pages.clear();
                                }
                                KeyCode::Char('q') | KeyCode::Char('Q') => break,
                                _ => {}
                            },
                            InputMode::EnteringTitle => match code {
                                KeyCode::Enter => {
                                    if !selector.new_book_title.is_empty() {
                                        selector.input_mode = InputMode::EnteringPages;
                                    }
                                }
                                KeyCode::Backspace => {
                                    selector.new_book_title.pop();
                                }
                                KeyCode::Esc => {
                                    selector.input_mode = InputMode::Selection;
                                }
                                KeyCode::Char(c) => {
                                    selector.new_book_title.push(c);
                                }
                                _ => {}
                            },
                            InputMode::EnteringPages => match code {
                                KeyCode::Enter => {
                                    if let Ok(pages) = selector.new_book_pages.parse::<usize>() {
                                        if pages > 0 {
                                            let book_index = add_book(selector.new_book_title.clone(), pages)?;
                                            library = load_library()?;
                                            selector.selected_index = book_index;
                                            selector.input_mode = InputMode::Selection;
                                        }
                                    }
                                }
                                KeyCode::Backspace => {
                                    selector.new_book_pages.pop();
                                }
                                KeyCode::Esc => {
                                    selector.input_mode = InputMode::Selection;
                                }
                                KeyCode::Char(c) if c.is_ascii_digit() => {
                                    selector.new_book_pages.push(c);
                                }
                                _ => {}
                            },
                        }
                    }
                }
            }
            AppMode::Reading => {
                if let Some(ref mut reading_app) = app {
                    terminal.draw(|f| ui::draw_ui(f, reading_app))?;

                    if event::poll(Duration::from_millis(100))? {
                        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                            match code {
                                KeyCode::Char(' ') => {
                                    if reading_app.timer_state == TimerState::Running {
                                        reading_app.pause();
                                    } else {
                                        reading_app.start();
                                    }
                                }
                                KeyCode::Char('s') | KeyCode::Char('S') => {
                                    reading_app.stop();
                                }
                                KeyCode::Char('b') | KeyCode::Char('B') => {
                                    if reading_app.timer_state != TimerState::Stopped {
                                        reading_app.stop();
                                    } else {
                                        reading_app.save_page();
                                    }
                                    library = load_library()?;
                                    app_mode = AppMode::BookSelection;
                                }
                                KeyCode::Char('q') | KeyCode::Char('Q') => {
                                    if reading_app.timer_state != TimerState::Stopped {
                                        reading_app.stop();
                                    } else {
                                        reading_app.save_page();
                                    }
                                    break;
                                }
                                KeyCode::Up => {
                                    reading_app.add_pages(10);
                                    reading_app.save_page();
                                }
                                KeyCode::Down => {
                                    reading_app.add_pages(-10);
                                    reading_app.save_page();
                                }
                                KeyCode::Right => {
                                    reading_app.increment_page();
                                    reading_app.save_page();
                                }
                                KeyCode::Left => {
                                    reading_app.decrement_page();
                                    reading_app.save_page();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
