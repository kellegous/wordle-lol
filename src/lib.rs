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

    pub fn contains(&self, c: Char) -> bool {
        self.chars.contains(&c)
    }

    pub fn to_string(&self) -> String {
        self.chars().into_iter().map(|c| c.char()).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Char> {
        self.chars().iter()
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

impl std::ops::Index<usize> for Word {
    type Output = Char;

    fn index(&self, ix: usize) -> &Self::Output {
        &self.chars[ix]
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

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Directive {
    Green,
    Yellow,
    Black,
}

impl Directive {
    fn to_ascii_char(&self) -> char {
        match self {
            Directive::Green => 'g',
            Directive::Yellow => 'y',
            Directive::Black => 'b',
        }
    }

    fn to_char(&self) -> char {
        match self {
            Directive::Green => '🟩',
            Directive::Yellow => '🟨',
            Directive::Black => '⬛',
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
pub struct Feedback {
    directives: [Directive; WORD_SIZE],
}

impl Feedback {
    pub fn from_str(s: &str) -> Result<Feedback, Box<dyn Error>> {
        if s.len() != WORD_SIZE {
            return Err(format!("feedback must be {} characters", WORD_SIZE).into());
        }
        let mut directives = [Directive::Green; WORD_SIZE];
        for (i, c) in s.char_indices() {
            directives[i] = match c {
                'g' | 'G' => Directive::Green,
                'y' | 'Y' => Directive::Yellow,
                'b' | 'B' => Directive::Black,
                _ => return Err(format!("invalid directive: {}", c).into()),
            };
        }
        Ok(Feedback { directives })
    }

    pub fn from_word(guess: &Word, solution: &Word) -> Feedback {
        let mut directives = [Directive::Black; WORD_SIZE];
        let mut claimed = [false; WORD_SIZE];

        // mark greens
        for (i, c) in guess.chars().iter().enumerate() {
            if solution[i] == *c {
                directives[i] = Directive::Green;
                claimed[i] = true;
            }
        }

        // mark yellows
        for (i, c) in guess.chars().iter().enumerate() {
            if directives[i] == Directive::Green {
                continue;
            }
            for (j, k) in solution.chars().iter().enumerate() {
                if !claimed[j] && *k == *c {
                    directives[i] = Directive::Yellow;
                    claimed[j] = true;
                    break;
                }
            }
        }

        Feedback { directives }
    }

    pub fn to_string(&self) -> String {
        self.directives().iter().map(|d| d.to_char()).collect()
    }

    pub fn to_ascii_string(&self) -> String {
        self.directives()
            .iter()
            .map(|d| d.to_ascii_char())
            .collect()
    }

    fn directives(&self) -> &[Directive] {
        &self.directives
    }

    pub fn is_all_green(&self) -> bool {
        self.directives.iter().all(|d| *d == Directive::Green)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Directive> {
        self.directives.iter()
    }
}

impl std::ops::Index<usize> for Feedback {
    type Output = Directive;

    fn index(&self, ix: usize) -> &Self::Output {
        &self.directives[ix]
    }
}

impl std::fmt::Display for Feedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
