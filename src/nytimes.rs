use std::error::Error;

use super::Word;
use chrono::NaiveDate;
use scraper::html::Html;
use scraper::Selector;
use url::Url;

const GAME_URL: &str = "https://www.nytimes.com/games/wordle/index.html";

const WORDLE_URL_PREFIX: &str = "wordle.";
const WORDLE_URL_SUFFIX: &str = ".js";

pub struct Data {
    version: String,
    solutions: Vec<Word>,
    guesses: Vec<Word>,
}

impl Data {
    fn fetch_and_parse(url: &str, version: String) -> Result<Data, Box<dyn Error>> {
        let data = reqwest::blocking::get(url)?.text()?;
        parse_words_from_script(&data, version)
    }

    pub fn fetch() -> Result<Data, Box<dyn Error>> {
        let (version, url) = find_wordle_script_url(&reqwest::blocking::get(GAME_URL)?.text()?)?;
        Self::fetch_and_parse(&url, version)
    }

    pub fn solutions(&self) -> &[Word] {
        &self.solutions
    }

    pub fn guesses(&self) -> &[Word] {
        &self.guesses
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn solution_on(&self, day: NaiveDate) -> (i64, Word) {
        let start = NaiveDate::from_ymd(2021, 6, 19);
        let ix = day.signed_duration_since(start).num_days();
        (ix, self.solutions[ix as usize % self.solutions.len()])
    }

    pub fn all(&self) -> impl Iterator<Item = &Word> {
        self.solutions.iter().chain(self.guesses.iter())
    }
}

fn parse_words_from_script(s: &str, version: String) -> Result<Data, Box<dyn Error>> {
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
        version: version,
        solutions: serde_json::from_str(&s[a..=a + b])?,
        guesses: serde_json::from_str(&s[c..=c + d])?,
    })
}

fn cut_after_last<'a>(sep: &'a str, s: &'a str) -> Option<&'a str> {
    if let Some(ix) = s.rfind(sep) {
        Some(&s[ix + 1..])
    } else {
        None
    }
}

fn parse_wordle_url(s: &str) -> Option<String> {
    if let Ok(url) = Url::parse(s) {
        if let Some(name) = cut_after_last("/", url.path()) {
            if name.starts_with(WORDLE_URL_PREFIX) && name.ends_with(WORDLE_URL_SUFFIX) {
                return Some(
                    name[WORDLE_URL_PREFIX.len()..name.len() - WORDLE_URL_SUFFIX.len()].to_owned(),
                );
            }
        }
    }
    None
}

fn find_wordle_script_url(s: &str) -> Result<(String, String), Box<dyn Error>> {
    let html = Html::parse_document(s);
    let selector = Selector::parse("script").unwrap();
    for el in html.select(&selector) {
        if let Some(src) = el.value().attr("src") {
            if let Some(version) = parse_wordle_url(src) {
                return Ok((version, src.to_owned()));
            }
        }
    }
    Err("wordle script url not found".into())
}
