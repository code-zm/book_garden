use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadingSession {
    pub book_title: String,
    pub start_page: usize,
    pub end_page: usize,
    pub duration_secs: u64,
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookProgress {
    pub book_title: String,
    pub total_pages: usize,
    pub current_page: usize,
    pub sessions: Vec<ReadingSession>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub books: Vec<BookProgress>,
}

const STORAGE_DIR: &str = "reading_data";
const LIBRARY_FILE: &str = "reading_data/library.json";

/// Initialize storage directory
pub fn init_storage() -> std::io::Result<()> {
    let path = Path::new(STORAGE_DIR);
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

/// Load library from file
pub fn load_library() -> std::io::Result<Library> {
    init_storage()?;

    let path = Path::new(LIBRARY_FILE);
    if path.exists() {
        let data = fs::read_to_string(path)?;
        match serde_json::from_str(&data) {
            Ok(library) => Ok(library),
            Err(_) => Ok(Library { books: Vec::new() }),
        }
    } else {
        Ok(Library { books: Vec::new() })
    }
}

/// Save library to file
pub fn save_library(library: &Library) -> std::io::Result<()> {
    init_storage()?;
    let json = serde_json::to_string_pretty(library)?;
    fs::write(LIBRARY_FILE, json)?;
    Ok(())
}

/// Add a new book to the library
pub fn add_book(book_title: String, total_pages: usize) -> std::io::Result<usize> {
    let mut library = load_library()?;

    library.books.push(BookProgress {
        book_title,
        total_pages,
        current_page: 0,
        sessions: Vec::new(),
    });

    let index = library.books.len() - 1;
    save_library(&library)?;
    Ok(index)
}

/// Update book in library
pub fn update_book(book_index: usize, progress: &BookProgress) -> std::io::Result<()> {
    let mut library = load_library()?;

    if book_index < library.books.len() {
        library.books[book_index] = progress.clone();
        save_library(&library)?;
    }

    Ok(())
}

/// Save reading session for a specific book
pub fn save_session(book_index: usize, session: &ReadingSession) -> std::io::Result<()> {
    let mut library = load_library()?;

    if book_index < library.books.len() {
        library.books[book_index].current_page = session.end_page;
        library.books[book_index].sessions.push(session.clone());
        save_library(&library)?;
    }

    Ok(())
}

/// Save just the current page without a session
pub fn save_current_page(book_index: usize, current_page: usize) -> std::io::Result<()> {
    let mut library = load_library()?;

    if book_index < library.books.len() {
        library.books[book_index].current_page = current_page;
        save_library(&library)?;
    }

    Ok(())
}

/// Get total reading statistics
pub fn get_statistics(progress: &BookProgress) -> (u64, usize) {
    let total_time: u64 = progress.sessions.iter().map(|s| s.duration_secs).sum();
    let total_sessions = progress.sessions.len();

    (total_time, total_sessions)
}
