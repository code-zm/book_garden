use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::storage::Library;

pub struct BookSelector {
    pub selected_index: usize,
    pub input_mode: InputMode,
    pub new_book_title: String,
    pub new_book_pages: String,
}

pub enum InputMode {
    Selection,
    EnteringTitle,
    EnteringPages,
}

impl BookSelector {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            input_mode: InputMode::Selection,
            new_book_title: String::new(),
            new_book_pages: String::new(),
        }
    }

    pub fn select_next(&mut self, library: &Library) {
        if library.books.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % library.books.len();
    }

    pub fn select_prev(&mut self, library: &Library) {
        if library.books.is_empty() {
            return;
        }
        if self.selected_index == 0 {
            self.selected_index = library.books.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }
}

pub fn draw_book_select(f: &mut Frame, library: &Library, selector: &BookSelector) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(5),     // Book list
            Constraint::Length(8),  // New book input
            Constraint::Length(4),  // Controls
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Book Library")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Book list
    let books: Vec<ListItem> = library
        .books
        .iter()
        .enumerate()
        .map(|(i, book)| {
            let progress = if book.total_pages > 0 {
                (book.current_page as f64 / book.total_pages as f64 * 100.0) as u32
            } else {
                0
            };

            let content = format!(
                "{} - {}/{} pages ({}%)",
                book.book_title, book.current_page, book.total_pages, progress
            );

            let style = if i == selector.selected_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let book_list = List::new(books)
        .block(Block::default().borders(Borders::ALL).title("Books"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(book_list, chunks[1]);

    // New book input
    let (title_color, pages_color) = match selector.input_mode {
        InputMode::EnteringTitle => (Color::Yellow, Color::White),
        InputMode::EnteringPages => (Color::White, Color::Yellow),
        InputMode::Selection => (Color::White, Color::White),
    };

    let new_book_block = Block::default().borders(Borders::ALL).title("Add New Book");
    let inner = new_book_block.inner(chunks[2]);
    f.render_widget(new_book_block, chunks[2]);

    let input_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    let title_input = Paragraph::new(format!("Title: {}", selector.new_book_title))
        .style(Style::default().fg(title_color));
    f.render_widget(title_input, input_chunks[0]);

    let pages_input = Paragraph::new(format!("Total Pages: {}", selector.new_book_pages))
        .style(Style::default().fg(pages_color));
    f.render_widget(pages_input, input_chunks[1]);

    // Controls
    let controls = match selector.input_mode {
        InputMode::Selection => {
            if library.books.is_empty() {
                "N: New Book | Q: Quit"
            } else {
                "↑/↓: Select | Enter: Open Book | N: New Book | Q: Quit"
            }
        }
        InputMode::EnteringTitle => "Type title, then Enter",
        InputMode::EnteringPages => "Type number of pages, then Enter",
    };

    let controls_widget = Paragraph::new(controls)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls_widget, chunks[3]);
}
