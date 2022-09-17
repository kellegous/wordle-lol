use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::Serialize;
use std::error::Error;

pub mod nytimes;

pub const WORD_SIZE: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Char {
    c: u8,
}

impl Char {
    pub fn from_char(c: char) -> Char {
        Char {
            c: c.to_ascii_lowercase() as u8,
        }
    }

    pub fn char(&self) -> char {
        self.c as char
    }
}

impl Default for Char {
    fn default() -> Char {
        Char { c: 'a' as u8 }
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Debug, PartialOrd, Eq, Ord)]
pub struct Word {
    chars: [Char; WORD_SIZE],
}

impl Word {
    pub fn from_str(s: &str) -> Result<Word, Box<dyn Error>> {
        if s.len() != WORD_SIZE {
            Err("word has to be 5 characters".into())
        } else {
            let mut chars = [Char::default(); WORD_SIZE];
            for (i, c) in s.char_indices() {
                chars[i] = Char::from_char(c)
            }
            Ok(Word { chars: chars })
        }
    }

    pub fn chars(&self) -> &[Char] {
        &self.chars
    }

    pub fn to_string(&self) -> String {
        self.chars().into_iter().map(|c| c.char()).collect()
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Serialize for Word {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Word {
    fn deserialize<D>(d: D) -> Result<Word, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_str(WordVisitor {})
    }
}
struct WordVisitor;

impl<'de> Visitor<'de> for WordVisitor {
    type Value = Word;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "5 character word")
    }

    fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Word::from_str(val) {
            Ok(word) => Ok(word),
            Err(e) => Err(E::custom(e)),
        }
    }
}
