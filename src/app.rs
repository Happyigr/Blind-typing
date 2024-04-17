use std::{
    fs::{self, File},
    io::{self, Write},
};

use crossterm::event::KeyModifiers;
use rand::{rngs::ThreadRng, Rng};
use ratatui::text::Line;

use self::typing_screen::{JSONResults, TypingMode};

pub mod typing_screen;

#[derive(Clone, Copy, PartialEq)]
pub enum Screens {
    Typing,
    TypingResult,
    GlobalResultMain,
    LetterResult,
    Exiting,
    Main,
    Alert,
}

impl Screens {
    pub fn as_title(&self) -> &str {
        match self {
            Screens::Typing => "Typing",
            Screens::TypingResult => "Typing Results",
            Screens::GlobalResultMain => "Global Typing Results",
            Screens::LetterResult => "Global Letter Result",
            Screens::Exiting => "Exit",
            Screens::Main => "Blind Typing",
            Screens::Alert => "TODO error mssg?",
        }
    }
    pub fn get_keys_hints(&self) -> &str {
        match self {
            Screens::Main => {
                "q - exit app, s - start, r - global results, R - delete existing result data"
            }
            Screens::Typing => "Esc - main screen, Tab - empty the typing",
            Screens::TypingResult => "q - main screen, c - continue typing",
            Screens::GlobalResultMain => {
                "letter - letter result, Esc - main screen, Tab - switch to big letters"
            }
            Screens::LetterResult => {
                "letter - another letter, Esc - global results, Tab - switch to big letters"
            }
            Screens::Exiting => "y - yes, n - no",
            Screens::Alert => "TODO ",
        }
    }
}

pub struct App {
    pub current_screen: Screens,
    pub pressed_letter: char,
    pub shift_pressed: bool,
    pub alert_text: String,
    pub previous_screen: Screens,
    file: Vec<String>,
    typing_mode: TypingMode,
    rand: ThreadRng,
}

impl App {
    pub fn new(filename: &str) -> App {
        let file = fs::read_to_string(filename).unwrap();

        App {
            current_screen: Screens::Main,
            previous_screen: Screens::Main,
            pressed_letter: ' ',
            shift_pressed: false,
            alert_text: "Init value".to_string(),
            typing_mode: TypingMode::new(),
            rand: rand::thread_rng(),
            file: file
                .split('\n')
                .filter(|l| l != &"")
                .map(|l| l.to_string())
                .collect(),
        }
    }

    pub fn alert(&mut self, text: &str) {
        self.alert_text = text.to_string();
        self.previous_screen = self.current_screen;
        self.current_screen = Screens::Alert;
    }
    pub fn reload_typing(&mut self) {
        self.typing_mode.reload_typing();
        self.shift_pressed = false;
    }

    pub fn start_typing(&mut self) {
        self.current_screen = Screens::Typing;
        let index = self.rand.gen_range(0..self.file.len());
        self.typing_mode.init(self.file[index].clone());
    }

    pub fn get_typing_text(&self) -> Line {
        self.typing_mode.get_text_to_render()
    }

    pub fn get_last_results(&self) -> &JSONResults {
        self.typing_mode.get_last_results()
    }

    pub fn delete_json(&self) {
        let mut file = File::create("src/results.json").unwrap();
        file.write_all("".as_bytes()).unwrap();
    }

    pub fn guess(&mut self, key: char) -> Option<bool> {
        self.typing_mode.guess(key)
        // cursor movement todo!
    }
}
