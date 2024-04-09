use std::{fs::File, io::BufReader};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{
    typing_screen::{JSONLetterInfo, JSONResults},
    App, Screens,
};

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
        Screens::TypingResult => render_results(f, &chunks[1], app.get_last_results()),
        Screens::GlobalResultMain => render_results(f, &chunks[1], &read_json_results_from_file()),
        Screens::LetterResult => render_letter_results(f, &chunks[1], app.letter_for_result),
        Screens::Exiting => (),
        Screens::Main => render_logo(f, &chunks[1]),
    };
}

fn render_logo(f: &mut Frame, area: &Rect) {
    let paragraph = Paragraph::new(Text::styled(
        "╔╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╗
╠╬╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╬╣
╠╣                                                       ╠╣
╠╣    ╠══╦╦══╣  ╔╗  ╔╗  ╔╦══╦╗  ╔╗  ╔╦╗   ╔╗   ╔═══╗     ╠╣
╠╣       ╠╣     ╠╣  ╠╣  ╠╣  ╠╣  ╚╝  ╠╣╚╗  ╠╣  ╔╝         ╠╣
╠╣       ╠╣     ╚╩══╬╣  ╠╬══╩╝  ╔╗  ╠╣ ╚╗ ╠╣  ║   ══╗    ╠╣
╠╣       ╠╣         ╠╣  ╠╣      ╠╣  ╠╣  ╚╗╠╣  ╚╗   ╔╝    ╠╣
╠╣       ╚╝     ╠═══╩╝  ╚╝      ╚╝  ╚╝   ╚╩╝   ╚═══╝     ╠╣
╠╣                                                       ╠╣
╠╣           ╠══╦╦══╣  ╔════╣  ╔═══╣   ╠══╦╦══╣          ╠╣
╠╣              ╠╣     ║       ║          ╠╣             ╠╣
╠╣              ╠╣     ╠════╣  ╚═══╗      ╠╣             ╠╣
╠╣              ╠╣     ║           ║      ╠╣             ╠╣
╠╣              ╚╝     ╚════╣  ╠═══╝      ╚╝             ╠╣
╠╣                                                       ╠╣
╠╬╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╬╣
╚╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╝",
        Style::new().fg(Color::Red),
    ))
    .centered();

    let chunks = Layout::vertical([Constraint::Min(1), Constraint::Min(20), Constraint::Min(1)])
        .split(*area);
    f.render_widget(paragraph, chunks[1]);
}

fn read_json_results_from_file() -> JSONResults {
    let file = File::open("src/results.json").unwrap();
    let read_buf = BufReader::new(file);
    let readed_json: JSONResults = serde_json::from_reader(read_buf).unwrap();

    readed_json
}

fn render_letter_results(f: &mut Frame, area: &Rect, ch: char) {
    let results_json = read_json_results_from_file();
    // todo make the error handling
    let letters_info = results_json.letters_info.get(&ch).unwrap();

    let time_line = Line::styled(
        format!(
            "Letter: \'{ch}\', Speed: todo()! wpm, Total accuracy: {}%",
            letters_info.letter_accuracies.get(&ch).unwrap()
        ),
        Style::new().fg(Color::Red),
    )
    .centered();

    let letters_line = Line::default()
        // iterating through the all info about letters and get the accuracy of the main letter
        .spans(letters_info.letter_accuracies.iter().map(|(ch, accuracy)| {
            let color = match accuracy {
                perc if *perc == 0.0 => Color::White,
                perc if *perc >= 80.0 => Color::Green,
                perc if *perc >= 50.0 => Color::Blue,
                perc if *perc <= 50.0 => Color::Red,
                _ => Color::White,
            };
            Span::styled(format!("{ch}:{accuracy}% "), Style::new().fg(color))
        }))
        .centered();

    let letters_paragraph = Paragraph::new(letters_line)
        .wrap(Wrap { trim: true })
        .centered();

    let main_chunk = Layout::vertical([
        Constraint::Length(5),
        Constraint::Percentage(100),
        Constraint::Length(11),
    ])
    .split(*area);
    let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[0]);

    f.render_widget(time_line, chunks[0]);
    f.render_widget(letters_paragraph, chunks[1]);

    let keyboard_chunk = Layout::horizontal([
        Constraint::Min(10),
        Constraint::Length(67),
        Constraint::Min(10),
    ])
    .split(main_chunk[2]);
    render_keyboard(f, &keyboard_chunk[1]);
}

fn render_results(f: &mut Frame, area: &Rect, results: &JSONResults) {
    let time_line = Line::styled(
        format!(
            "Speed: {} wpm, Total accuracy: {}%",
            results.wpm, results.total_accuracy
        ),
        Style::new().fg(Color::Red),
    )
    .centered();

    let letters_line = Line::default()
        // iterating through the all info about letters and get the accuracy of the main letter
        .spans(results.letters_info.iter().map(|(ch, letter_info)| {
            let accuracy = letter_info.letter_accuracies.get(&ch).unwrap_or(&0.0);
            let color = match accuracy {
                perc if *perc == 0.0 => Color::White,
                perc if *perc >= 80.0 => Color::Green,
                perc if *perc >= 50.0 => Color::Blue,
                perc if *perc <= 50.0 => Color::Red,
                _ => Color::White,
            };
            Span::styled(format!("{ch}:{accuracy}% "), Style::new().fg(color))
        }))
        .centered();

    let letters_paragraph = Paragraph::new(letters_line)
        .wrap(Wrap { trim: true })
        .centered();

    let main_chunk = Layout::vertical([
        Constraint::Length(5),
        Constraint::Percentage(100),
        Constraint::Length(11),
    ])
    .split(*area);
    let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[0]);

    f.render_widget(time_line, chunks[0]);
    f.render_widget(letters_paragraph, chunks[1]);

    let keyboard_chunk = Layout::horizontal([
        Constraint::Min(10),
        Constraint::Length(67),
        Constraint::Min(10),
    ])
    .split(main_chunk[2]);
    render_keyboard(f, &keyboard_chunk[1]);
}

fn render_keyboard(f: &mut Frame, area: &Rect) {
    let main_block = Block::bordered().border_type(BorderType::Rounded);
    let inner_area = main_block.inner(*area);
    let inner_chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .split(inner_area);

    render_word_in_blocks("qwertyuiop[]\\", &inner_chunks[0], f, 0);
    render_word_in_blocks("asdfghjkl;'", &inner_chunks[1], f, 5);
    render_word_in_blocks("zxcvbnm,./", &inner_chunks[2], f, 7);

    f.render_widget(main_block, *area);
}

// this renders every letter of a word in one block
fn render_word_in_blocks(letters: &str, chunk: &Rect, f: &mut Frame, margin: u16) {
    let area = Layout::horizontal(letters.chars().into_iter().map(|_| Constraint::Length(5)))
        .horizontal_margin(margin)
        .split(*chunk);
    for (i, ch) in letters.chars().into_iter().enumerate() {
        render_str_in_block(&ch.to_string(), &area[i], f);
    }
}

fn render_str_in_block(text: &str, area: &Rect, f: &mut Frame) {
    let block = Block::bordered().border_type(BorderType::Rounded);
    let to_render = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(to_render, *area);
}
