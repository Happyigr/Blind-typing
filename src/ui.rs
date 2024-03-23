use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Screens};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        app.current_screen.as_title(),
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Center)
    .block(title_block);

    let key_hints = app.current_screen.get_keys_hints();

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let footer = Paragraph::new(Text::styled(
        app.current_screen.get_keys_hints(),
        Style::default().fg(Color::White),
    ))
    .alignment(Alignment::Center)
    .block(footer_block);

    f.render_widget(title, chunks[0]);
    f.render_widget(footer, chunks[2]);

    let main_part = match app.current_screen {
        Screens::Typing => app.get_typing_text(),
        Screens::TypingResult => Line::default(),
        Screens::GlobalResultMain => Line::default(),
        Screens::LetterResult => Line::default(),
        Screens::Exiting => Line::default(),
        Screens::Main => Line::default(),
    }
    .alignment(Alignment::Center);

    f.render_widget(main_part, chunks[1]);
}
