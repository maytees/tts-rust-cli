extern crate dotenv;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use dotenv::dotenv;
use rusty_audio::Audio;
use std::env;

const SPEECH_SPEED: &str = "-2";
const OUTPUT_CODEC: &str = "mp3";
const SPEECH_FORMAT: &str = "8khz_8bit_mono";
const LANGAUGE: &str = "en-us";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please provide args - Message for Text To Speech");
    }

    let message = match args.get(1) {
        Some(msg) => msg,
        None => panic!("Could not get message from input"),
    };

    let mut rapidhost = String::from("");
    let mut rapidtoken = String::from("");
    let mut voicetoken = String::from("");

    for (key, value) in env::vars() {
        match key.as_str() {
            "RAPIDHOST" => rapidhost = value,
            "RAPIDKEY" => rapidtoken = value,
            "VOICEKEY" => voicetoken = value,
            _ => (),
        }
    }

    let client = reqwest::Client::new();

    let link = format!(
        "https://voicerss-text-to-speech.p.rapidapi.com/?key={}&src={}&hl={}&r={}&c={}&f={}",
        voicetoken, message, LANGAUGE, SPEECH_SPEED, OUTPUT_CODEC, SPEECH_FORMAT
    );

    let res = client
        .get(link)
        .header("X-RapidAPI-Key", rapidtoken)
        .header("X-RapidAPI-Host", rapidhost)
        .send()
        .await?
        .bytes()
        .await?;

    let mut ttsfile = File::create("tts.mp3").expect("Error!");
    ttsfile.write_all(&res).expect("ERror writing!");

    let mut audio = Audio::new();

    audio.add("tts", "tts.mp3");
    audio.play("tts");
    audio.wait();

    fs::remove_file("tts.mp3").expect("Could not remove file!");

    Ok(())
}
