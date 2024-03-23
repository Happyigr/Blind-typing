mod result_screen;
use result_screen::Accuracy;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
};

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub struct TypingMode {
    current_text: String,
    correct_letter: char,
    last_guessed: bool,
    guessed: usize,
    attempts: usize,
    results: HashMap<char, Accuracy>,
}

impl TypingMode {
    pub fn new() -> TypingMode {
        TypingMode {
            current_text: "init value".to_string(),
            correct_letter: 'i',
            last_guessed: true,
            guessed: 0,
            attempts: 0,
            results: HashMap::new(),
        }
    }

    pub fn init(&mut self, text: String) {
        self.guessed = 0;
        self.attempts = 0;
        self.last_guessed = true;
        self.current_text = text;
        self.results = HashMap::new();
        // this will always be a letter, because we have only &str that are not empty
        self.correct_letter = self.current_text.chars().nth(self.guessed).unwrap();
    }

    pub fn reload_typing(&mut self) {
        self.attempts = 0;
        self.guessed = 0;
        self.last_guessed = true;
        self.results = HashMap::new();
        self.correct_letter = self.current_text.chars().nth(self.guessed).unwrap();
    }

    pub fn guess(&mut self, key: char) -> Result<Option<bool>, io::Error> {
        self.attempts += 1;
        // if user typed right letter
        if key == self.correct_letter {
            // if there are some letter in word
            if let Some(ch) = self.current_text.chars().nth(self.guessed + 1) {
                // insert the info about typed letter in results hashmap
                if let Some(pair) = self.results.get_mut(&self.correct_letter) {
                    pair.attempts += self.attempts;
                    pair.amount += 1;
                } else {
                    self.results
                        .insert(self.correct_letter, Accuracy::new(1, self.attempts));
                }
                self.attempts = 0;
                self.correct_letter = ch;
                self.last_guessed = true;
                self.guessed += 1;
                Ok(Some(true))
            // if not return the Err(())
            } else {
                self.result_calculation()?;
                Ok(None)
            }
        // if the user typed wrong letter
        } else {
            self.last_guessed = false;
            Ok(Some(false))
        }
    }

    // this function writes the results in the json file
    fn result_calculation(&self) -> Result<(), io::Error> {
        let mut results_vec: Vec<(char, f64)> = vec![];

        for (k, v) in self.results.iter() {
            results_vec.push((*k, v.get_percent()));
        }

        let results_json = serde_json::to_string(&results_vec).expect("Failed to parse the json");

        let mut file = File::create("src/results.json")?;
        file.write_all(results_json.as_bytes())?;

        Ok(())
    }

    pub fn get_text_to_render(&self) -> Line {
        let mut parts: Vec<Span> = vec![];

        // the guessed part, that should be green
        parts.push(Span::styled(
            // from start guessed letter forward
            &self.current_text[..self.guessed],
            Style::default().fg(Color::Green),
        ));

        // if the last guess was wrong it will be equal to 1 otherwise 0
        let guess_bit = if self.last_guessed { 0 } else { 1 };

        // the part that should be red
        parts.push(Span::styled(
            // RevertAddBool adds + 1 if its false
            // only one letter, if the last guess was wrong
            &self.current_text[self.guessed..self.guessed + guess_bit],
            Style::default().fg(Color::Red),
        ));

        // the default text
        parts.push(Span::styled(
            // all from guessed letters and + 1 if the last was wrong
            &self.current_text[self.guessed + guess_bit..],
            Style::default().fg(Color::White),
        ));

        Line::from(parts)
    }
}
