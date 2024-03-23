use std::fs;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
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

    match app.current_screen {
        Screens::Typing => {
            let main_part = app.get_typing_text().alignment(Alignment::Center);
            f.render_widget(main_part, chunks[1]);
        }
        Screens::TypingResult => {
            let main_chunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(10), Constraint::Length(11)])
                .split(chunks[1]);
            let keyboard_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Length(52),
                    Constraint::Min(10),
                ])
                .split(main_chunk[1]);
            render_results();
            let main_part = Line::from(Span::styled(
                fs::read_to_string("src/results.json").unwrap(),
                Style::default().fg(Color::White),
            ));
            f.render_widget(main_part, main_chunk[0]);
            render_keyboard(f, keyboard_chunk[1]);
        }
        Screens::GlobalResultMain => (),
        Screens::LetterResult => (),
        Screens::Exiting => (),
        Screens::Main => (),
    };
}

fn render_results() {
    todo!();
}

fn render_keyboard(f: &mut Frame, area: Rect) {
    let main_block = Block::default().borders(Borders::ALL);
    let inner_area = main_block.inner(area);
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(inner_area);
    render_row("qwertyuiop", inner_chunks[0], f, 0);
    render_row("asdfghjkl", inner_chunks[1], f, 2);
    render_row("zxcvbnm", inner_chunks[2], f, 6);

    f.render_widget(main_block, area);
}

fn render_row(letters: &str, chunk: Rect, f: &mut Frame, margin: usize) {
    let letters = letters;
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(letters.chars().into_iter().map(|_| Constraint::Length(5)))
        .horizontal_margin(margin as u16)
        .split(chunk);
    for (i, ch) in letters.chars().into_iter().enumerate() {
        f.render_widget(get_button(ch), area[i]);
    }
}

fn get_button<'a>(letter: char) -> Paragraph<'a> {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let keycap = Paragraph::new(letter.to_string())
        .alignment(Alignment::Center)
        .block(block);

    keycap
}
