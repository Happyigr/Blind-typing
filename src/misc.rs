use std::{fs::{self, File}, hash::{BuildHasher, DefaultHasher}, io::{self, Read}};

use chatgpt::{client::ChatGPT, types::CompletionResponse};
use ratatui::style::Color;

#[derive(Default)]
pub struct MyHasher {}

impl MyHasher {
    pub fn new() -> MyHasher {
        MyHasher {}
    }
}

impl BuildHasher for MyHasher {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        DefaultHasher::new()
    }
}

pub fn get_color_by_accuracy(accuracy: f64) -> Color {
    match accuracy {
        perc if perc == 101.0 => Color::Yellow,
        perc if perc == 0.0 => Color::Reset,
        perc if perc >= 80.0 => Color::Green,
        perc if perc >= 50.0 => Color::Blue,
        perc if perc <= 50.0 => Color::Red,
        _ => Color::Reset,
    }
}

pub async fn get_chatgpt_words() -> Result<String, chatgpt::err::Error> {
    // Getting the API key here
    let key = fs::read_to_string("src/api_key.txt")?;
    let key = key[..key.len()-1].to_string();

    // Creating a new ChatGPT client.
    // Note that it requires an API key, and uses
    // tokens from your OpenAI API account balance.
    let client = ChatGPT::new(key)?;

    // Sending a message and getting the completion
    let response: CompletionResponse = client
        .send_message("Write me 10 sentences, separated with newline and are good for blind typing test, but not the default examples. Write nothing else but the sentences without the numbers")
        .await?;

   Ok(response.message().content.to_string()) 

}
