use std::fs;

use rand::{rngs::ThreadRng, Rng};
use ratatui::text::Line;

use self::typing_mode::TypingMode;

mod typing_mode;
pub struct App {
    pub current_screen: Screens,
    // file: Vec<String>,
    typing_mode: TypingMode,
    rand: ThreadRng,
}

pub enum Screens {
    Typing,
    TypingResult,
    GlobalResultMain,
    LetterResult,
    Exiting,
    Main,
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
        }
    }
    pub fn get_keys_hints(&self) -> &str {
        match self {
            Screens::Main => "q - exit app, s - start, r - global results",
            Screens::Typing => "Esc - main screen, r - empty the typing",
            Screens::TypingResult => "q - main screen, c - continue typing",
            Screens::GlobalResultMain => "letter - letter result, Esc - main screen",
            Screens::LetterResult => "letter - another letter, Esc - global results",
            Screens::Exiting => "y - yes, n - no",
        }
    }
}

impl App {
    pub fn new(filename: &str) -> App {
        // let file = fs::read_to_string(filename).unwrap();

        App {
            current_screen: Screens::Main,
            typing_mode: TypingMode::new(),
            rand: rand::thread_rng(),
            // file: file
            //     .split('\n')
            //     .filter(|l| l != &"")
            //     .map(|l| l.to_string())
            //     .collect(),
        }
    }

    pub fn reload_typing(&mut self) {
        self.typing_mode.reload_typing();
    }

    pub fn start_typing(&mut self) {
        self.current_screen = Screens::Typing;
        // let index = self.rand.gen_range(0..self.file.len());
        self.typing_mode.init("Hello World");
    }

    pub fn get_typing_text(&self) -> Line {
        self.typing_mode.get_text_to_render()
    }

    pub fn guess(&mut self, key: char) -> Option<bool> {
        self.typing_mode.guess(key)
        // cursor movement todo!
    }
}
