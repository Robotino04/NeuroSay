use rand::seq::IteratorRandom;
use serde::Deserialize;

static RANTS_DATABASE_URL: &str = "https://rants.theharrisontemple.com/data.json";

#[derive(Deserialize, Debug)]
struct RantText {
    pub speaker: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
struct Rant {
    pub date: u64,
    pub tags: Vec<String>,
    pub text: Vec<RantText>,
    pub author: String,
    pub id: String,
}

fn main() {
    let all_rants = reqwest::blocking::get(RANTS_DATABASE_URL)
        .expect("Couldn't connect to database.")
        .json::<Vec<Rant>>()
        .expect("Couldn't parse database response");
    let rant = all_rants
        .iter()
        .choose(&mut rand::thread_rng())
        .expect("No rants match the filters");

    let prefix_length = rant
        .text
        .iter()
        .map(|text| text.speaker.len())
        .max()
        .unwrap_or(0);

    for text in &rant.text {
        print!("[{}] ", text.speaker);

        let color = match text.speaker.to_lowercase().as_str() {
            "neuro" => "\x1b[38;2;255;192;203m",
            "vedal" => "\x1b[38;2;50;185;69m",
            "evil" => "\x1b[38;2;255;51;51m",
            "tts" => "\x1b[38;2;100;65;165m",
            "collab partner" => "\x1b[38;2;18;105;227m",
            _ => "",
        };

        println!(
            "{color}{}\x1b[0m",
            textwrap::indent(
                textwrap::fill(text.text.as_str(), 60).as_str(),
                &" ".repeat(prefix_length + 3)
            )
            .trim_end()
            .split_at(text.speaker.len() + 3)
            .1
        );
    }
}
