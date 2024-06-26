use chrono::{DateTime, Local};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, ErrorKind, Write},
};

use crate::misc::MyHasher;

struct LetterInfo {
    presses: usize,
    pressed_letters: HashMap<char, usize>,
}

impl LetterInfo {
    fn new(pressed_key: char) -> LetterInfo {
        let mut blob = LetterInfo {
            presses: 0,
            pressed_letters: HashMap::new(),
        };

        blob.insert_press(pressed_key);

        blob
    }

    fn insert_press(&mut self, ch: char) {
        self.presses += 1;

        if let Some(amount) = self.pressed_letters.get_mut(&ch) {
            *amount += 1;
        } else {
            self.pressed_letters.insert(ch, 1);
        }
    }

    // i dont need to store the main letter in the structure, because i have this letter in the
    // hashmap of typinginfo
    fn to_json(&self, main_letter: char) -> JSONLetterInfo {
        let mut letter_presses = HashMap::with_hasher(MyHasher::new());

        for (ch, guesses) in self.pressed_letters.iter() {
            // let accuracy = Accuracy::new(self.presses, *guesses);
            letter_presses.insert(*ch, *guesses);
        }

        JSONLetterInfo {
            main_letter,
            letter_accuracies: letter_presses,
            presses_of_key: self.presses,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JSONLetterInfo {
    // pub wpm: f64,
    pub main_letter: char,
    // containes the letters and the accuracy of those letters
    pub letter_accuracies: HashMap<char, usize, MyHasher>,
    presses_of_key: usize,
}

impl JSONLetterInfo {
    pub fn get_perc(&self, ch: char) -> f64 {
        ((*self.letter_accuracies.get(&ch).unwrap() as f64 / self.presses_of_key as f64) * 1000.0)
            .round()
            / 10.0
    }

    fn update(&mut self, other: &JSONLetterInfo) {
        // every letter that were tapped
        self.presses_of_key = self.presses_of_key + other.presses_of_key;
        for (ch, other) in other.letter_accuracies.iter() {
            if let Some(main) = self.letter_accuracies.get_mut(ch) {
                *main = *main + *other;
            } else {
                self.letter_accuracies.insert(*ch, *other);
            }
        }
    }

    fn get_copy(&self) -> JSONLetterInfo {
        let copy = self
            .letter_accuracies
            .iter()
            .map(|(ch, acc)| (*ch, acc.clone()))
            .collect::<HashMap<char, usize, MyHasher>>();
        JSONLetterInfo {
            main_letter: self.main_letter,
            letter_accuracies: copy,
            presses_of_key: self.presses_of_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JSONResults {
    pub wpm: f64,
    pub total_accuracy: f64,
    pub letters_info: HashMap<char, JSONLetterInfo, MyHasher>,
}

impl JSONResults {
    pub fn get_total_results(&self) -> HashMap<char, f64, MyHasher> {
        self.letters_info
            .iter()
            .map(|(ch, letter_info)| (*ch, letter_info.get_perc(*ch)))
            .collect::<HashMap<char, f64, MyHasher>>()
    }
    pub fn get_result_by_letter(
        &self,
        pressed_ch: char,
    ) -> Result<HashMap<char, f64, MyHasher>, io::Error> {
        let letter_info = if let Some(letter_info) = self.letters_info.get(&pressed_ch) {
            letter_info
        } else {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "We have no info about this letter :(",
            ));
        };

        let results: HashMap<char, f64, MyHasher> = letter_info
            .letter_accuracies
            .iter()
            .map(|(ch, _)| (*ch, letter_info.get_perc(*ch)))
            .collect();

        Ok(results)
    }

    fn new() -> JSONResults {
        JSONResults {
            wpm: 0.0,
            total_accuracy: 0.0,
            letters_info: HashMap::with_hasher(MyHasher::new()),
        }
    }

    fn update(&mut self, other: &JSONResults) {
        if self.wpm != 0.0 {
            self.wpm = (self.wpm + other.wpm) / 2.0;
        } else {
            self.wpm = other.wpm
        }
        if self.total_accuracy != 0.0 {
            self.total_accuracy =
                ((self.total_accuracy + other.total_accuracy) / 2.0 * 10.0).round() / 10.0;
        } else {
            self.total_accuracy = other.total_accuracy;
        }

        for (ch, info_other) in other.letters_info.iter() {
            if let Some(info_main) = self.letters_info.get_mut(&ch) {
                info_main.update(&info_other);
            } else {
                self.letters_info.insert(*ch, info_other.get_copy());
            }
        }
    }
}

pub struct TypingMode {
    start_time: Option<DateTime<Local>>,
    current_text: String,

    presses: usize,
    correct_letter: char,
    last_guessed: bool,
    guessed_letters: usize,
    results: HashMap<char, LetterInfo>,

    result_data: Option<JSONResults>,
}

impl TypingMode {
    pub fn new() -> TypingMode {
        TypingMode {
            current_text: "init value".to_string(),
            correct_letter: 'i',
            last_guessed: true,
            guessed_letters: 0,
            presses: 0,
            start_time: None,
            result_data: None,
            results: HashMap::new(),
        }
    }

    pub fn init(&mut self, text: String) {
        self.current_text = text;
        self.result_data = None;
        self.reload_typing();
    }

    pub fn reload_typing(&mut self) {
        self.presses = 0;
        self.guessed_letters = 0;
        self.last_guessed = true;
        // this will always be a letter, because we have only &str that are not empty
        self.correct_letter = self.current_text.chars().nth(self.guessed_letters).unwrap();
        self.start_time = None;
        self.results = HashMap::new();
    }

    pub fn guess(&mut self, pressed_key: char) -> Option<bool> {
        if self.start_time.is_none() {
            self.start_time = Some(Local::now());
        }

        self.presses += 1;
        if let Some(pair) = self.results.get_mut(&self.correct_letter) {
            pair.insert_press(pressed_key);
        } else {
            self.results
                .insert(self.correct_letter, LetterInfo::new(pressed_key));
        }

        // if user typed right letter
        if pressed_key == self.correct_letter {
            // if there are some letter in word
            if let Some(ch) = self.current_text.chars().nth(self.guessed_letters + 1) {
                // self.attempts = 0;
                self.correct_letter = ch;
                self.last_guessed = true;
                self.guessed_letters += 1;
                Some(true)
            // if there are no letters more return the Err(())
            } else {
                self.guessed_letters += 1;
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
        let file =
            File::open("results.json").unwrap_or_else(|_| File::create("results.json").unwrap());
        let read_buf = BufReader::new(file);
        let mut readed_json: JSONResults =
            serde_json::from_reader(read_buf).unwrap_or(JSONResults::new());

        let typing_time = Local::now().signed_duration_since(self.start_time.unwrap());

        let words: Vec<&str> = self.current_text.split_whitespace().collect();
        let wpm = (words.len() as f64 / typing_time.num_milliseconds() as f64 * 60000.0 * 10.0)
            .round()
            / 10.0;

        let total_accuracy =
            ((self.guessed_letters as f64 / self.presses as f64) * 1000.0).round() / 10.0;

        let letters_info: HashMap<char, JSONLetterInfo, MyHasher> = self
            .results
            .iter_mut()
            .map(|(ch, letter_info)| (*ch, letter_info.to_json(*ch)))
            .collect();

        let new_json: JSONResults = JSONResults {
            wpm,
            total_accuracy,
            letters_info,
        };

        readed_json.update(&new_json);
        self.result_data = Some(new_json);

        let mut file = File::create("results.json").unwrap();
        file.write_all(serde_json::to_string(&readed_json).unwrap().as_bytes())
            .unwrap();
    }

    pub fn get_last_results(&self) -> &JSONResults {
        self.result_data.as_ref().unwrap()
    }

    pub fn get_text_to_render(&self) -> Line {
        let mut parts: Vec<Span> = vec![];

        // the guessed part, that should be green
        parts.push(Span::styled(
            // from start guessed letter forward
            &self.current_text[..self.guessed_letters],
            Style::default().fg(Color::Green),
        ));

        // if the last guess was wrong it will be equal to 1 otherwise 0
        let guess_bit = if self.last_guessed { 0 } else { 1 };

        // the part that should be red
        parts.push(Span::styled(
            // RevertAddBool adds + 1 if its false
            // only one letter, if the last guess was wrong
            &self.current_text[self.guessed_letters..self.guessed_letters + guess_bit],
            Style::default().fg(Color::Red),
        ));

        // the default text
        parts.push(Span::styled(
            // all from guessed letters and + 1 if the last was wrong
            &self.current_text[self.guessed_letters + guess_bit..],
            Style::default().fg(Color::White),
        ));

        Line::from(parts)
    }
}
