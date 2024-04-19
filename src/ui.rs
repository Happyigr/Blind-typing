use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
    Frame,
};
use std::{collections::HashMap, fs::File, io::BufReader, rc::Rc};

use crate::app::{typing_screen::JSONResults, App, Screens};

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

            let mut tapped_letter = HashMap::new();
            tapped_letter.insert(app.get_pressed_letter(), 101.0);

            f.render_stateful_widget(
                Keyboard,
                layout[1],
                &mut KeyboardState::new(tapped_letter, app.get_uppercase()),
            );
            f.render_widget(main_part, layout[0]);
        }
        Screens::TypingResult => {
            render_results(
                f,
                &chunks[1],
                app.get_uppercase(),
                None,
                Some(app.get_last_results()),
            );
        }
        Screens::GlobalResultMain => render_results(f, &chunks[1], app.get_uppercase(), None, None),
        Screens::LetterResult => {
            // todo! make the hashmap always equal see: hasher in rust
            render_results(
                f,
                &chunks[1],
                app.get_uppercase(),
                Some(app.get_pressed_letter()),
                None,
            )
        }
        Screens::Exiting => {}
        Screens::Main => render_logo(f, &chunks[1]),
        Screens::Alert => {
            // alert(f, &app.alert_text)
        }
    };
}
fn alert(f: &mut Frame, text: &str) {
    let alert_chunk = Layout::vertical([
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
    .split(alert_chunk[1])[1];

    let bg_block = Block::default()
        .title_top("Error")
        .borders(Borders::NONE)
        .bg(Color::Black);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .bg(Color::DarkGray)
        .fg(Color::Red);

    let paragraph = Paragraph::new(Text::styled(text, Style::new().fg(Color::Red))).block(block);

    f.render_widget(Clear, f.size());
    f.render_widget(bg_block, alert_chunk);
    f.render_widget(paragraph, alert_chunk);
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

fn render_results(
    f: &mut Frame,
    area: &Rect,
    uppercase: bool,
    choosed_letter: Option<char>,
    typing_results: Option<&JSONResults>,
) {
    let json_results = {
        let file = File::open("src/results.json").unwrap();
        let read_buf = BufReader::new(file);
        let json_results: JSONResults = serde_json::from_reader(read_buf).unwrap();
        json_results
    };
    let json_results = match typing_results {
        Some(res) => res,
        None => &json_results,
    };

    // if there are letter choosen, then it is the results from one letter
    let results = match choosed_letter {
        Some(ch) => json_results.get_result_by_letter(ch),
        None => json_results.get_total_results(),
    };
    let total_accuracy = match choosed_letter {
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

    let mut keyboard_state = KeyboardState::new(results, uppercase);

    f.render_widget(main_info, upper_chunks[0]);
    f.render_widget(letters_block, letters_chunk);
    f.render_stateful_widget(Keyboard, main_chunk[2], &mut keyboard_state)
}

struct Keyboard;
struct KeyboardState {
    keys_to_highlight: HashMap<char, f64>,
    uppercase: bool,
}

impl KeyboardState {
    fn new(keys_to_highlight: HashMap<char, f64>, uppercase: bool) -> KeyboardState {
        KeyboardState {
            keys_to_highlight,
            uppercase,
        }
    }
}

impl StatefulWidget for Keyboard {
    type State = KeyboardState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let letters = match state.uppercase {
            false => "qwertyuiop[]\\asdfghjkl;\'zxcvbnm,./ ",
            true => "QWERTYUIOP{}|ASDFGHJKL:\"ZXCVBNM<>? ",
        };

        let letters = letters
            .chars()
            .into_iter()
            .map(|ch| {
                let accuracy = *state.keys_to_highlight.get(&ch).unwrap_or(&0.0);
                Keycap {
                    ch,
                    color: get_color_by_accuracy(accuracy),
                }
            })
            .collect::<Vec<Keycap>>();

        let keyboard_chunk = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(67),
            Constraint::Fill(1),
        ])
        .split(area)[1];

        let keyboard_rows = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .flex(Flex::Center)
        .split(keyboard_chunk);

        let rows_lengths: [usize; 4] = [13, 11, 10, 1];
        // building the layout of the rows
        let keyboard_rows = keyboard_rows
            .into_iter()
            .enumerate()
            .map(|(row_i, row)| {
                // if this the 1,2,3 rows it will be a row with the length 5 so many charachtecrs
                // long, as it in rows_lengths under the 0,1,2 places
                // on the 4 row it will be a place for space 48 symbols long
                Layout::horizontal(vec![
                    match row_i {
                        3 => Constraint::Length(48),
                        _ => Constraint::Length(5),
                    };
                    rows_lengths[row_i]
                ])
                .flex(Flex::Center)
                .split(*row)
            })
            .collect::<Vec<Rc<[Rect]>>>();

        // first row 13 ch, second 11 ch, third 10 ch, and space
        for (key_i, keycap) in letters.into_iter().enumerate() {
            match key_i {
                key_i if key_i <= 12 => keycap.render(keyboard_rows[0][key_i], buf),
                key_i if key_i <= 23 => keycap.render(keyboard_rows[1][key_i - 13], buf),
                key_i if key_i <= 33 => keycap.render(keyboard_rows[2][key_i - 24], buf),
                _ => keycap.render(keyboard_rows[3][0], buf),
            }
        }

        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(keyboard_chunk, buf);
    }
}

struct Keycap {
    ch: char,
    color: Color,
}

impl Widget for Keycap {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.ch.to_string())
            .style(Style::new().fg(self.color))
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(area, buf);
    }
}

fn get_color_by_accuracy(accuracy: f64) -> Color {
    match accuracy {
        perc if perc == 101.0 => Color::Yellow,
        perc if perc == 0.0 => Color::Reset,
        perc if perc >= 80.0 => Color::Green,
        perc if perc >= 50.0 => Color::Blue,
        perc if perc <= 50.0 => Color::Red,
        _ => Color::Reset,
    }
}
