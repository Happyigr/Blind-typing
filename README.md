![main_screen.png](main_screen.png)

# Blind typing Test

Hi there, I have tryed to improve my blind typing powers, and have found some usefull websites, but theyy were not perfect for me. Because of that I have made my own blind typing app in Rust.

## Installation

Build this by yourself(you need a rust language installed):
- `git clone https://github.com/Happyigr/Blind-typing`
- `cd in the folder of the project`
- `cargo run`

Or download the release and unzip it, there is the executable, that you can run(maybe `chmod +x blind_typing` will be needed).

If you want to make new texts with chatgpt, you have to write your api key in the file `api_key.txt` under the src directory or in the directory of the executable.

## Why rust?

I have heard a lot about rust, and I wanted to learn it. I have read the Rust book, but i want to tr, to implement smth useful. Because of it I wanted to make my own project, that i can show on my job or my friends.

## Plans

I want to implement both of versions of my app: TUI and GUI (with tauri).

- [x] - make the working terminal app, with all the screens
- [x] - the text to it (hints for keys) and the movement from screen to screen.
- [x] - the main logic of the app
- [x] - results after typing
- [x] - better results after typing (no JSON)
- [x] - text getting from other resources
- [x] - dynimacly results when typing with colored keyboard, that shows taps on keyboard
- [x] - global results of all the typing tests
- [x] - global results by letter
- [ ] - better ui
- [x] - error handling

GUI app:
- [ ] - make the working website with tauri
- [ ] - Main, Results, Typing screens
- [ ] - typing process
- [ ] - results after typing
- [ ] - dynamic results in typing mode
- [ ] - global results
- [ ] - results for every letter

## Thanks for

- ChatGPT :)
- https://docs.rs/ratatui/latest/ratatui/index.html
- https://www.keybr.com/
- https://doc.rust-lang.org/stable/book/
