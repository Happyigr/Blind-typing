use chrono::{DateTime, Local};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Write};

#[derive(Deserialize, Serialize)]
pub struct ResultData {
    pub words_amount: usize,
    pub time: usize,
    pub letters: HashMap<char, f64>,
}

#[derive(Clone, Copy)]
pub struct Accuracy {
    // amount of letters in the text
    pub amount: usize,
    // attempts of typing the letter
    pub attempts: usize,
}

impl Accuracy {
    pub fn new(amount: usize, attempts: usize) -> Accuracy {
        Accuracy { amount, attempts }
    }

    pub fn get_percent(&self) -> f64 {
        ((self.amount as f64 / self.attempts as f64) * 1000.0).round() / 10.0
    }
}

pub struct TypingMode {
    current_text: String,
    correct_letter: char,
    last_guessed: bool,
    guessed: usize,
    attempts: usize,
    results: HashMap<char, Accuracy>,
    start_time: Option<DateTime<Local>>,
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
            start_time: None,
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
        self.start_time = None;
    }

    pub fn reload_typing(&mut self) {
        self.attempts = 0;
        self.guessed = 0;
        self.last_guessed = true;
        self.results = HashMap::new();
        // this will always be a letter, because we have only &str that are not empty
        self.correct_letter = self.current_text.chars().nth(self.guessed).unwrap();
        self.start_time = None;
    }

    pub fn guess(&mut self, key: char) -> Option<bool> {
        if self.start_time.is_none() {
            self.start_time = Some(Local::now());
        }
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
                Some(true)
            // if not return the Err(())
            } else {
                self.result_calculation();
                None
            }
        // if the user typed wrong letter
        } else {
            self.last_guessed = false;
            Some(false)
        }
    }

    // this function writes the results in the json file
    fn result_calculation(&mut self) {
        let typing_time = Local::now().signed_duration_since(self.start_time.unwrap());
        let mut letters_results: HashMap<char, f64> = HashMap::new();
        let words: Vec<&str> = self.current_text.split_whitespace().collect();

        for (ch, acc) in self.results.iter_mut() {
            letters_results.insert(*ch, acc.get_percent());
        }

        let result_json: ResultData = ResultData {
            words_amount: words.len(),
            time: typing_time.num_milliseconds() as usize,
            letters: letters_results,
        };

        let mut file = File::create("src/results.json").unwrap();
        file.write_all(serde_json::to_string(&result_json).unwrap().as_bytes())
            .unwrap();
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
