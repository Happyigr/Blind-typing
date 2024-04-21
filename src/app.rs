use std::{
    fs::{self, File},
    io::Write,
};

use rand::{rngs::ThreadRng, Rng};
use ratatui::text::Line;

use crate::misc::get_chatgpt_words;

use self::typing_screen::{JSONResults, TypingMode};

pub mod typing_screen;

#[derive(Clone, Copy, PartialEq, Debug)]
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
                "q - exit app, s - start, r - global results, R - delete existing result data, t - get new texts"
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
    file: Vec<String>,
    events: AppEvents,
    typing_mode: TypingMode,
    rand: ThreadRng,
}

struct AppEvents {
    current_screen: Screens,
    previous_screen: Screens,
    pressed_letter: char,
    is_uppercase: bool,
    alert_text: String,
}

impl AppEvents {
    fn new() -> AppEvents {
        AppEvents {
            current_screen: Screens::Main,
            previous_screen: Screens::Main,
            pressed_letter: ' ',
            is_uppercase: false,
            alert_text: "init value".to_string(),
        }
    }

    fn change_screen_to(&mut self, new_screen: Screens) {
        if new_screen != self.current_screen {
            self.previous_screen = self.current_screen;
            self.current_screen = new_screen;
        }
    }
}

impl App {
    pub fn new(filename: &str) -> App {
        let file = fs::read_to_string(filename).unwrap();

        App {
            events: AppEvents::new(),
            typing_mode: TypingMode::new(),
            rand: rand::thread_rng(),
            file: file
                .split('\n')
                .filter(|l| l != &"")
                .map(|l| l.to_string())
                .collect(),
        }
    }

    pub fn set_key_pressed(&mut self, ch: char) {
        self.events.pressed_letter = ch;
    }

    pub fn set_uppercase(&mut self, getted: bool) {
        self.events.is_uppercase = getted;
    }

    pub fn change_uppercase(&mut self) {
        self.events.is_uppercase = !self.events.is_uppercase;
    }

    pub fn change_screen(&mut self, new_screen: Screens) {
        self.events.change_screen_to(new_screen);
    }

    pub fn get_uppercase(&self) -> bool {
        self.events.is_uppercase
    }

    pub fn get_alert_text(&self) -> &str {
        &self.events.alert_text
    }

    pub fn get_pressed_letter(&self) -> char {
        self.events.pressed_letter
    }

    pub fn get_previous_screen(&self) -> Screens {
        self.events.previous_screen
    }

    pub fn get_current_screen(&self) -> &Screens {
        &self.events.current_screen
    }

    pub fn get_typing_text(&self) -> Line {
        self.typing_mode.get_text_to_render()
    }

    pub fn get_last_results(&self) -> &JSONResults {
        self.typing_mode.get_last_results()
    }

    pub fn alert(&mut self, text: String) {
        self.events.alert_text = text;
        self.events.current_screen = Screens::Alert;
    }

    pub fn guess(&mut self) -> Option<bool> {
        self.typing_mode.guess(self.events.pressed_letter)
        // blanked cursore todo!
    }

    pub fn reload_typing(&mut self) {
        self.typing_mode.reload_typing();
        self.events.is_uppercase = false;
    }

    pub fn start_typing(&mut self) {
        self.change_screen(Screens::Typing);
        let index = self.rand.gen_range(0..self.file.len());
        self.typing_mode.init(self.file[index].clone());
    }

    pub async fn get_new_texts(&mut self) {
        let words = get_chatgpt_words().await.unwrap();
        let mut file = File::create("src/texts.txt").unwrap();
        file.write_all(words.as_bytes()).unwrap();
    }

    pub fn delete_json(&self) {
        let mut file = File::create("src/results.json").unwrap();
        file.write_all("".as_bytes()).unwrap();
    }
}
