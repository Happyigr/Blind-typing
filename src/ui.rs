use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
};

use crate::{
    app::{typing_screen::JSONResults, App, Screens},
    misc::get_color_by_accuracy,
};
use crate::{misc::MyHasher, widgets::keyboard::*};

pub fn ui(f: &mut Frame, app: &mut App) {
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
        app.get_current_screen().as_title(),
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Center)
    .block(title_block);

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let footer = Paragraph::new(Text::styled(
        app.get_current_screen().get_keys_hints(),
        Style::default().fg(Color::White),
    ))
    .alignment(Alignment::Center)
    .block(footer_block);

    f.render_widget(title, chunks[0]);
    f.render_widget(footer, chunks[2]);

    match app.get_current_screen() {
        Screens::Typing => {
            let main_part = app.get_typing_text().alignment(Alignment::Center);
            let layout = Layout::vertical([Constraint::Percentage(50), Constraint::Length(14)])
                .split(chunks[1]);

            let mut tapped_letter = HashMap::with_hasher(MyHasher::new());
            tapped_letter.insert(app.get_pressed_letter(), 101.0);

            f.render_stateful_widget(
                Keyboard,
                layout[1],
                &mut KeyboardState::new(tapped_letter, app.get_uppercase()),
            );
            f.render_widget(main_part, layout[0]);
        }
        Screens::TypingResult => {
            if let Err(err) = render_results(
                f,
                &chunks[1],
                app.get_uppercase(),
                None,
                Some(app.get_last_results()),
            ) {
                app.alert(err.to_string());
                alert(f, app);
            }
        }
        Screens::GlobalResultMain => {
            if let Err(err) = render_results(f, &chunks[1], app.get_uppercase(), None, None) {
                app.alert(err.to_string());
                alert(f, app);
            }
        }
        Screens::LetterResult => {
            if let Err(err) = render_results(
                f,
                &chunks[1],
                app.get_uppercase(),
                Some(app.get_pressed_letter()),
                None,
            ) {
                app.alert(err.to_string());
                alert(f, app);
            }
        }
        Screens::Exiting => {}
        Screens::Main => {
            render_logo(f, &chunks[1]);
        }
        Screens::Alert => alert(f, app),
    };
}

fn alert(f: &mut Frame, app: &mut App) {
    let main_chunks = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Min(1),
        Constraint::Percentage(20),
    ])
    .split(f.size());

    let alert_chunk = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Min(1),
        Constraint::Percentage(30),
    ])
    .split(main_chunks[1]);

    let block = Block::bordered()
        .title_top("Error")
        .border_type(BorderType::Rounded)
        .bg(Color::Black);

    let err_text = Paragraph::new(Text::styled(
        format!(
            "\n\nErr: {}\n\nTap any letter to exit this window.",
            app.get_alert_text()
        ),
        Style::new().fg(Color::Red),
    ))
    .wrap(Wrap { trim: true })
    .alignment(Alignment::Center)
    .block(block);

    f.render_widget(Clear, f.size());
    f.render_widget(err_text, alert_chunk[1]);
}

// todo rewrite as widget
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

// todo rewrite as widget
fn render_results(
    f: &mut Frame,
    area: &Rect,
    is_uppercase: bool,
    choosed_letter: Option<char>,
    typing_results: Option<&JSONResults>,
) -> Result<(), io::Error> {
    let json_results = {
        let file = File::open("results.json")?;
        let read_buf = BufReader::new(file);
        let json_results: JSONResults = serde_json::from_reader(read_buf)?;
        json_results
    };

    let json_results = match typing_results {
        Some(res) => res,
        None => &json_results,
    };

    // if there are letter choosen, then it is the results from one letter
    let mut results = match choosed_letter {
        Some(ch) => match json_results.get_result_by_letter(ch) {
            Ok(results) => results,
            Err(err) => return Err(err),
        },
        None => json_results.get_total_results(),
    };

    let total_accuracy = match choosed_letter {
        // if there are a result of one letter tha, we need to take the accuracy of this letter,
        // otherwise total accuracy
        //
        // here will be always a value, because the letter info always contains the info about
        // the main letter if there are results about it
        Some(ch) => json_results.letters_info.get(&ch).unwrap().get_perc(ch),
        None => json_results.total_accuracy,
    };

    let wpm = json_results.wpm;

    let main_info = Line::styled(
        format!(
            "Speed: {} wpm, Total accuracy: {}%",
            (wpm * 100.0).round() / 100.0,
            total_accuracy
        ),
        Style::new().fg(Color::Red),
    )
    .centered();

    // collecting all the results
    let mut letters_line = results
        .iter()
        .map(|(ch, accuracy)| (*ch, *accuracy))
        .collect::<Vec<(char, f64)>>();

    // ascending sort of the vector
    letters_line.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut letters_line = Line::default()
        // iterating through the all info about letters and get the accuracy of the main letter
        .spans(letters_line.iter().map(|(ch, accuracy)| {
            if let Some(ch_l) = choosed_letter {
                if *ch == ch_l {
                    return Span::default();
                }
            }
            let color = get_color_by_accuracy(*accuracy);
            Span::styled(format!("{ch}:{accuracy}% "), Style::new().fg(color))
        }))
        .centered();

    if letters_line.spans.len() == 1 {
        letters_line = Line::default().spans([Span::styled(
            "You made it to the 100% percent accuracy!",
            Style::default(),
        )
        .fg(Color::Green)]);
    }

    let letters_block = Paragraph::new(letters_line)
        .wrap(Wrap { trim: true })
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Letters Info")
                .title_alignment(Alignment::Center),
        )
        .fg(Color::White);

    let main_chunk = Layout::vertical([
        Constraint::Length(9),
        Constraint::Min(1),
        Constraint::Length(14),
    ])
    .split(*area);

    let upper_chunks =
        Layout::vertical([Constraint::Length(3), Constraint::Percentage(100)]).split(main_chunk[0]);

    let letters_chunk = Layout::horizontal([Constraint::Min(1)])
        .horizontal_margin(5)
        .split(upper_chunks[1])[0];

    let mut keyboard_state = match choosed_letter {
        Some(ch) => {
            results.insert(ch, 101.0);
            KeyboardState::new(results, is_uppercase)
        }
        None => KeyboardState::new(results, is_uppercase),
    };

    f.render_widget(main_info, upper_chunks[0]);
    f.render_widget(letters_block, letters_chunk);
    f.render_stateful_widget(Keyboard, main_chunk[2], &mut keyboard_state);

    Ok(())
}
