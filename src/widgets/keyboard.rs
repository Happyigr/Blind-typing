use std::{collections::HashMap, rc::Rc};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::misc::MyHasher;

pub struct Keyboard;
pub struct KeyboardState {
    keys_to_highlight: HashMap<char, f64, MyHasher>,
    uppercase: bool,
}

impl KeyboardState {
    pub fn new(keys_to_highlight: HashMap<char, f64, MyHasher>, uppercase: bool) -> KeyboardState {
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
