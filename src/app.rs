use std::time::{Duration, Instant};
use crate::storage::{ReadingSession, save_session, save_current_page, BookProgress};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug)]
pub struct AppState {
    pub timer_state: TimerState,
    pub start_time: Option<Instant>,
    pub elapsed_before_pause: Duration,
    pub current_page: usize,
    pub total_pages: usize,
    pub book_title: String,
    pub session_start_page: usize,
    pub total_time_secs: u64,
    pub total_sessions: usize,
    pub book_index: usize,
}

impl AppState {
    pub fn new(book_index: usize, progress: &BookProgress) -> Self {
        let (total_time_secs, total_sessions) = crate::storage::get_statistics(progress);

        Self {
            timer_state: TimerState::Stopped,
            start_time: None,
            elapsed_before_pause: Duration::ZERO,
            current_page: progress.current_page,
            total_pages: progress.total_pages,
            book_title: progress.book_title.clone(),
            session_start_page: progress.current_page,
            total_time_secs,
            total_sessions,
            book_index,
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.timer_state {
            TimerState::Running => {
                self.elapsed_before_pause + self.start_time.map_or(Duration::ZERO, |s| s.elapsed())
            }
            TimerState::Paused | TimerState::Stopped => self.elapsed_before_pause,
        }
    }

    pub fn start(&mut self) {
        if self.timer_state != TimerState::Running {
            self.timer_state = TimerState::Running;
            self.start_time = Some(Instant::now());
            self.session_start_page = self.current_page;
        }
    }

    pub fn pause(&mut self) {
        if self.timer_state == TimerState::Running {
            self.elapsed_before_pause += self.start_time.unwrap().elapsed();
            self.start_time = None;
            self.timer_state = TimerState::Paused;
        }
    }

    pub fn stop(&mut self) {
        if self.timer_state == TimerState::Running {
            self.pause();
        }

        // Save session if there was any time spent
        if !self.elapsed_before_pause.is_zero() {
            let session = ReadingSession {
                book_title: self.book_title.clone(),
                start_page: self.session_start_page,
                end_page: self.current_page,
                duration_secs: self.elapsed_before_pause.as_secs(),
                timestamp: chrono::Local::now(),
            };

            if let Err(e) = save_session(self.book_index, &session) {
                eprintln!("Error saving session: {}", e);
            } else {
                // Update total time and sessions count
                self.total_time_secs += self.elapsed_before_pause.as_secs();
                self.total_sessions += 1;
            }
        } else {
            // If no time was spent, just save the current page
            if let Err(e) = save_current_page(self.book_index, self.current_page) {
                eprintln!("Error saving current page: {}", e);
            }
        }

        self.timer_state = TimerState::Stopped;
        self.start_time = None;
        self.elapsed_before_pause = Duration::ZERO;
        self.session_start_page = self.current_page;
    }

    pub fn save_page(&self) {
        if let Err(e) = save_current_page(self.book_index, self.current_page) {
            eprintln!("Error saving current page: {}", e);
        }
    }

    pub fn load_book(&mut self, book_index: usize, progress: &BookProgress) {
        let (total_time_secs, total_sessions) = crate::storage::get_statistics(progress);

        self.book_index = book_index;
        self.book_title = progress.book_title.clone();
        self.total_pages = progress.total_pages;
        self.current_page = progress.current_page;
        self.session_start_page = progress.current_page;
        self.total_time_secs = total_time_secs;
        self.total_sessions = total_sessions;

        // Reset timer state when switching books
        self.timer_state = TimerState::Stopped;
        self.start_time = None;
        self.elapsed_before_pause = Duration::ZERO;
    }

    pub fn increment_page(&mut self) {
        if self.current_page < self.total_pages {
            self.current_page += 1;
        }
    }

    pub fn decrement_page(&mut self) {
        if self.current_page > 0 {
            self.current_page = self.current_page.saturating_sub(1);
        }
    }

    pub fn add_pages(&mut self, delta: i32) {
        let new_page = (self.current_page as i32 + delta).clamp(0, self.total_pages as i32);
        self.current_page = new_page as usize;
    }

    pub fn progress(&self) -> f64 {
        if self.total_pages == 0 {
            0.0
        } else {
            (self.current_page as f64 / self.total_pages as f64).min(1.0)
        }
    }

    pub fn pages_read_this_session(&self) -> i32 {
        self.current_page as i32 - self.session_start_page as i32
    }
}
