use std::error::Error;
use std::path::PathBuf;

use super::Word;
use chrono::NaiveDate;
use scraper::html::Html;
use scraper::Selector;
use url::Url;

const GAME_URL: &str = "https://www.nytimes.com/games/wordle/index.html";

pub struct Data {
    solutions: Vec<Word>,
    guesses: Vec<Word>,
}

impl Data {
    fn fetch_and_parse(url: &str) -> Result<Data, Box<dyn Error>> {
        let data = reqwest::blocking::get(url)?.text()?;
        parse_words_from_script(&data)
    }

    pub fn fetch() -> Result<Data, Box<dyn Error>> {
        let url = find_wordle_script_url(&reqwest::blocking::get(GAME_URL)?.text()?)?;
        Self::fetch_and_parse(&url)
    }

    pub fn solutions(&self) -> &[Word] {
        &self.solutions
    }

    pub fn guesses(&self) -> &[Word] {
        &self.guesses
    }

    pub fn solution_on(&self, day: NaiveDate) -> Word {
        let start = NaiveDate::from_ymd(2021, 6, 19);
        let ix = day.signed_duration_since(start).num_days();
        self.solutions[ix as usize % self.solutions.len()]
    }
}

fn parse_words_from_script(s: &str) -> Result<Data, Box<dyn Error>> {
    let a = match s.find(r#"["cigar",""#) {
        Some(ix) => ix,
        None => return Err("could not find prefix of solution array".into()),
    };
    let b = match s[a..].find("]") {
        Some(ix) => ix,
        None => return Err("could not find suffix of solutions array".into()),
    };

    let c = match s.find(r#"["aahed",""#) {
        Some(ix) => ix,
        None => return Err("could not find prefix of guesses array".into()),
    };
    let d = match s[c..].find("]") {
        Some(ix) => ix,
        None => return Err("could not find suffix of guesses array".into()),
    };

    Ok(Data {
        solutions: serde_json::from_str(&s[a..=a + b])?,
        guesses: serde_json::from_str(&s[c..=c + d])?,
    })
}

fn is_wordle_url(s: &str) -> bool {
    match Url::parse(s) {
        Ok(u) => {
            if let Some(name) = PathBuf::from(u.path()).file_name() {
                name.to_str().unwrap_or("").starts_with("wordle.")
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

fn find_wordle_script_url(s: &str) -> Result<String, Box<dyn Error>> {
    let html = Html::parse_document(s);
    let selector = Selector::parse("script").unwrap();
    for el in html.select(&selector) {
        if let Some(src) = el.value().attr("src") {
            if is_wordle_url(src) {
                return Ok(src.to_owned());
            }
        }
    }
    Err("wordle script url not found".into())
}
