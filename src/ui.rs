use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::app::{AppState, TimerState};
use crate::plant::create_plant_canvas;

pub fn draw_ui(f: &mut Frame, app: &AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Legend bar
        ])
        .split(f.area());

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(65), // Left side: info
            Constraint::Percentage(35), // Right side: plant
        ])
        .split(main_chunks[0]);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Timer
            Constraint::Length(3), // Page counter
            Constraint::Length(3), // Progress bar
            Constraint::Min(5),    // Stats
        ])
        .split(content_chunks[0]);

    // Title
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            &app.book_title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title("Book Tracker"));

    f.render_widget(title, chunks[0]);

    // Timer
    let elapsed = app.elapsed();
    let hours = elapsed.as_secs() / 3600;
    let minutes = (elapsed.as_secs() % 3600) / 60;
    let seconds = elapsed.as_secs() % 60;

    let timer_text = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    let timer_color = match app.timer_state {
        TimerState::Running => Color::Green,
        TimerState::Paused => Color::Yellow,
        TimerState::Stopped => Color::Red,
    };

    let timer_status = match app.timer_state {
        TimerState::Running => "READING",
        TimerState::Paused => "PAUSED",
        TimerState::Stopped => "STOPPED",
    };

    let timer = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("{} ", timer_status),
            Style::default()
                .fg(timer_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            timer_text,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title("Time"));

    f.render_widget(timer, chunks[1]);

    // Page counter
    let pages_text = format!("Page {} of {}", app.current_page, app.total_pages);
    let pages = Paragraph::new(pages_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .style(Style::default().fg(Color::White));

    f.render_widget(pages, chunks[2]);

    // Progress bar
    let progress = app.progress();
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Completion"))
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .percent((progress * 100.0) as u16)
        .label(format!("{:.1}%", progress * 100.0));

    f.render_widget(gauge, chunks[3]);

    // Session stats
    let pages_this_session = app.pages_read_this_session();
    let total_hours = app.total_time_secs / 3600;
    let total_minutes = (app.total_time_secs % 3600) / 60;

    let stats_text = vec![
        Line::from(vec![
            Span::styled("Total time: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}h {}m", total_hours, total_minutes),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  Sessions: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.total_sessions),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled("Pages this session: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", pages_this_session),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Pages remaining: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.total_pages - app.current_page),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let stats = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"))
        .alignment(Alignment::Center);

    f.render_widget(stats, chunks[4]);

    // Plant canvas on the right side
    let progress = app.progress();
    let plant = create_plant_canvas(progress)
        .block(Block::default().borders(Borders::ALL).title("Growth"));
    f.render_widget(plant, content_chunks[1]);

    // Legend bar at the bottom
    let legend = Line::from(vec![
        Span::raw(" "),
        Span::styled("Space", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Start/Pause  "),
        Span::styled("S", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Stop  "),
        Span::styled("↑/↓", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" ±10 pages  "),
        Span::styled("→/←", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" ±1 page  "),
        Span::styled("B", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Book Select  "),
        Span::styled("Q", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Quit"),
    ]);

    let legend_widget = Paragraph::new(legend)
        .style(Style::default().bg(Color::DarkGray));

    f.render_widget(legend_widget, main_chunks[1]);
}
