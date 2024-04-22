mod app;
mod misc;
mod ui;
mod widgets;

use app::{App, Screens};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;
use ui::ui;

// this is clap
// This is what will be printed with help
/// Simple program to greet a person
// #[derive(Parser, Debug)]
// help is included automaticly and the version will print the version of the app out of the
// cargo.toml file
// #[command(version)]
// struct Args {
// this is what will be printed in help to this arg(name)
// Name of the person to greet
// this tells that we have both the short and long type of attribute in the cli app
// #[arg(short, long)]
// key: String,

// Number of times to greet
// without the default value the user must write it be yourself
// #[arg(short, long, default_value_t = 1)]
// count: u8,
// }

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let filename = "texts.txt";
    let mut app = App::new(filename);
    // run_app(&mut terminal, &mut app)?;
    run_app(&mut terminal, &mut app).await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

// running the main loop of the app
async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                break;
            }
            if let KeyCode::Char(ch) = key.code {
                app.set_key_pressed(ch);
            }

            if key.code == KeyCode::Tab {
                app.change_uppercase();
            }

            match app.get_current_screen() {
                Screens::Main => main_behavior(&key, app).await,
                Screens::Typing => typing_behavior(&key, app),
                Screens::Exiting => {
                    if exiting_behavior(&key, app) {
                        break;
                    }
                }
                Screens::TypingResult => end_typing_behaviour(&key, app),
                Screens::GlobalResultMain => global_res_behavior(&key, app),
                Screens::LetterResult => letter_res_behavior(&key, app),
                Screens::Alert => alert_behaviour(&key, app),
            }
        }
    }
    Ok(())
}

fn alert_behaviour(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char(_) => app.change_screen(app.get_previous_screen()),
        _ => (),
    }
}

async fn main_behavior(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') => app.change_screen(Screens::Exiting),
        KeyCode::Char('r') => app.change_screen(Screens::GlobalResultMain),
        KeyCode::Char('s') => app.start_typing(),
        KeyCode::Char('R') => app.delete_json(),
        KeyCode::Char('t') => {
            if let Err(err) = app.get_new_texts().await {
                app.alert(format!(
                    "You need to add the file with yor api key in the folder of the project, with the name \"api_key.txt\" if you want to use this feature.\n\n{}",
                    err.to_string(),
                ));
            }
        }
        _ => (),
    }
}
fn typing_behavior(key: &KeyEvent, app: &mut App) {
    app.set_uppercase(key.modifiers == KeyModifiers::SHIFT);

    match key.code {
        KeyCode::Esc => app.change_screen(Screens::Main),
        // reload the typing letters
        KeyCode::Tab => app.reload_typing(),
        KeyCode::Char(_) => {
            // if there are the next letter
            if let Some(_letter) = app.guess() {
                // else if there are no more letters in the word we end to typing
            } else {
                // make the end of the typing
                app.change_screen(Screens::TypingResult);
            }
        }
        _ => (),
    }
}
fn end_typing_behaviour(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') => app.change_screen(Screens::Main),
        KeyCode::Char('c') => app.start_typing(),
        _ => (),
    }
}
fn exiting_behavior(key: &KeyEvent, app: &mut App) -> bool {
    loop {
        match key.code {
            KeyCode::Char('y') => return true,
            KeyCode::Char('n') => {
                app.change_screen(Screens::Main);
                return false;
            }
            _ => (),
        }
    }
}
fn global_res_behavior(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc => app.change_screen(Screens::Main),
        KeyCode::Char(_) => app.change_screen(Screens::LetterResult),
        _ => (),
    }
}
fn letter_res_behavior(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc => app.change_screen(Screens::GlobalResultMain),
        KeyCode::Char(ch) => {
            app.set_key_pressed(ch);
            app.change_screen(Screens::LetterResult)
        }
        _ => (),
    }
}
