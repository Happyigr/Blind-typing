use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Styled, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use std::{collections::HashMap, fs::File, io::BufReader};

use crate::app::{typing_screen::JSONResults, App, Screens};

struct LetterInfo(char, f64);

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
            let layout = Layout::vertical([Constraint::Percentage(50), Constraint::Length(14)])
                .split(chunks[1]);
            let mut res = HashMap::new();
            res.insert(app.pressed_letter, 50.0);
            let keyboard_chunk = Layout::horizontal([
                Constraint::Min(1),
                Constraint::Length(67),
                Constraint::Min(1),
            ])
            .split(layout[1]);
            render_colored_keyboard(f, &keyboard_chunk[1], &res, false);
            f.render_widget(main_part, layout[0]);
        }
        Screens::TypingResult => {
            // render_results(f, &chunks[1], app.get_last_results(), app.shift_pressed)
            let json_results = app.get_last_results();

            let results = json_results
                .letters_info
                .iter()
                .map(|(ch, letter_info)| (*ch, *letter_info.letter_accuracies.get(&ch).unwrap()))
                .collect::<HashMap<char, f64>>();

            render_results(
                f,
                &chunks[1],
                &results,
                json_results.wpm,
                json_results.total_accuracy,
                app.shift_pressed,
            )
        }
        Screens::GlobalResultMain => {
            let json_results = read_json_results_from_file();

            let results = json_results
                .letters_info
                .iter()
                .map(|(ch, letter_info)| (*ch, *letter_info.letter_accuracies.get(&ch).unwrap()))
                .collect::<HashMap<char, f64>>();

            render_results(
                f,
                &chunks[1],
                &results,
                json_results.wpm,
                json_results.total_accuracy,
                app.shift_pressed,
            )
        }
        Screens::LetterResult => {
            let json_results = read_json_results_from_file();

            let results = &json_results
                .letters_info
                .get(&app.pressed_letter)
                .unwrap()
                .letter_accuracies;

            render_results(
                f,
                &chunks[1],
                &results,
                json_results.wpm,
                json_results.total_accuracy,
                app.shift_pressed,
            )
        }
        Screens::Exiting => alert(f, "hi"),
        Screens::Main => render_logo(f, &chunks[1]),
    };
}
fn alert(f: &mut Frame, text: &str) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .set_style(Style::new().fg(Color::LightMagenta).bg(Color::Red));
    let paragraph = Paragraph::new(Text::styled(text, Style::new().fg(Color::Red))).block(block);

    f.render_widget(paragraph, f.size())
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

fn render_results(
    f: &mut Frame,
    area: &Rect,
    results: &HashMap<char, f64>,
    wpm: f64,
    total_accuracy: f64,
    uppercase: bool,
) {
    let time_line = Line::styled(
        format!("Speed: {} wpm, Total accuracy: {}%", wpm, total_accuracy),
        Style::new().fg(Color::Red),
    )
    .centered();

    let letters_line = Line::default()
        // iterating through the all info about letters and get the accuracy of the main letter
        .spans(results.iter().map(|(ch, accuracy)| {
            let color = get_color_by_accuracy(*accuracy);
            Span::styled(format!("{ch}:{accuracy}% "), Style::new().fg(color))
        }))
        .centered();

    let letters_paragraph = Paragraph::new(letters_line)
        .wrap(Wrap { trim: true })
        .centered();

    let main_chunk = Layout::vertical([
        Constraint::Length(5),
        Constraint::Percentage(100),
        Constraint::Length(14),
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
    render_colored_keyboard(f, &keyboard_chunk[1], results, uppercase);
}

fn render_colored_keyboard(
    f: &mut Frame,
    area: &Rect,
    results: &HashMap<char, f64>,
    uppercasse: bool,
) {
    let main_block = Block::bordered().border_type(BorderType::Rounded);
    let inner_area = main_block.inner(*area);
    let inner_chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .split(inner_area);
    if uppercasse {
        render_word_in_blocks_colored("QWERTYUIOP{}|", &inner_chunks[0], f, 0, results);
        render_word_in_blocks_colored("ASDFGHJKL:\"", &inner_chunks[1], f, 5, results);
        render_word_in_blocks_colored("ZXCVBNM<>?", &inner_chunks[2], f, 7, results);
        // todo space rendering
    } else {
        render_word_in_blocks_colored("qwertyuiop[]\\", &inner_chunks[0], f, 0, results);
        render_word_in_blocks_colored("asdfghjkl;'", &inner_chunks[1], f, 5, results);
        render_word_in_blocks_colored("zxcvbnm,./", &inner_chunks[2], f, 7, results);
    }
    let space_chunks = Layout::horizontal([Constraint::Min(100)])
        .horizontal_margin(9)
        .split(inner_chunks[3]);
    let space = Block::bordered()
        .border_type(BorderType::Rounded)
        .style(get_color_by_accuracy(*results.get(&' ').unwrap_or(&0.0)));
    f.render_widget(space, space_chunks[0]);

    f.render_widget(main_block, *area);
}

fn render_word_in_blocks_colored(
    letters: &str,
    chunk: &Rect,
    f: &mut Frame,
    margin: u16,
    results: &HashMap<char, f64>,
) {
    let area = Layout::horizontal(letters.chars().into_iter().map(|_| Constraint::Length(5)))
        .horizontal_margin(margin)
        .split(*chunk);

    for (i, ch) in letters.chars().into_iter().enumerate() {
        let accuracy: f64 = *results.get(&ch).unwrap_or(&0.0);
        let color = get_color_by_accuracy(accuracy);

        let block = Block::bordered().border_type(BorderType::Rounded);
        let to_render = Paragraph::new(ch.to_string())
            .style(Style::new().fg(color))
            .alignment(Alignment::Center)
            .block(block);

        f.render_widget(to_render, area[i]);
    }
}

fn get_color_by_accuracy(accuracy: f64) -> Color {
    match accuracy {
        perc if perc == 0.0 => Color::Reset,
        perc if perc >= 80.0 => Color::Green,
        perc if perc >= 50.0 => Color::Blue,
        perc if perc <= 50.0 => Color::Red,
        _ => Color::Reset,
    }
}
