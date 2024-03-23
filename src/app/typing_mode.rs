use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub struct TypingMode {
    current_text: String,
    correct_letter: char,
    last_guessed: bool,
    guessed: usize,
}

impl<'a> TypingMode {
    pub fn new() -> TypingMode {
        TypingMode {
            current_text: "init value".to_string(),
            correct_letter: 'i',
            last_guessed: true,
            guessed: 0,
        }
    }

    pub fn init(&mut self, text: String) {
        self.guessed = 0;
        self.last_guessed = true;
        self.current_text = text;
        // this will always be a letter, because we have only &str that are not empty
        self.correct_letter = self.current_text.chars().nth(self.guessed).unwrap();
    }

    pub fn reload_typing(&mut self) {
        self.guessed = 0;
        self.last_guessed = true;
        self.correct_letter = self.current_text.chars().nth(self.guessed).unwrap();
    }

    pub fn guess(&mut self, key: char) -> Option<bool> {
        // if user typed right letter
        if key == self.correct_letter {
            // if there are some letter in word
            if let Some(ch) = self.current_text.chars().nth(self.guessed + 1) {
                self.correct_letter = ch;
                self.last_guessed = true;
                self.guessed += 1;
                Some(true)
            // if not return the Err(())
            } else {
                None
            }
        // if the user typed wrong letter
        } else {
            self.last_guessed = false;
            Some(false)
        }
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
