use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::Serialize;
use std::collections::{BTreeSet, HashMap};
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
        self.directives.iter().map(|d| d.to_char()).collect()
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

#[derive(Copy, Clone)]
pub enum Match {
    Is(Directive),
    IsNot(Directive),
}

impl Match {
    pub fn matches(&self, d: Directive) -> bool {
        match self {
            Self::Is(x) => d == *x,
            Self::IsNot(x) => d != *x,
        }
    }
}

pub struct Matcher {
    matches: [Match; WORD_SIZE],
}

impl Matcher {
    pub fn new(a: Match, b: Match, c: Match, d: Match, e: Match) -> Matcher {
        Matcher {
            matches: [a, b, c, d, e],
        }
    }

    pub fn matches(&self, feedback: &Feedback) -> bool {
        self.matches
            .iter()
            .zip(feedback.iter())
            .all(|(m, d)| m.matches(*d))
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Constraint {
    s: String,
}

impl Constraint {
    pub fn from_word_and_feedback(w: &Word, f: &Feedback) -> Constraint {
        let s = f
            .iter()
            .zip(w.iter())
            .filter_map(|(d, c)| {
                if *d == Directive::Yellow {
                    Some(c.char())
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>()
            .iter()
            .collect::<String>();
        Constraint { s }
    }

    pub fn is_compatible(&self, c: &Constraint) -> bool {
        c.len() > self.len() && self.s.chars().all(|v| c.s.contains(v))
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn empty() -> Constraint {
        Constraint { s: String::new() }
    }
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

fn next_group_of_guesses<'a>(
    m: &'a HashMap<Constraint, Vec<(Word, Feedback)>>,
    key: &Constraint,
) -> Option<(&'a Constraint, &'a Vec<(Word, Feedback)>)> {
    m.iter()
        .filter(|(k, _)| key.is_compatible(k))
        .max_by(|(_, a), (_, b)| b.len().cmp(&a.len()))
}

pub fn find_guesses<'a>(
    words: impl Iterator<Item = &'a Word>,
    solution: &Word,
    matcher: &Matcher,
    k: usize,
) -> Vec<(Word, Feedback)> {
    let selected = words.filter_map(|w| {
        let f = Feedback::from_word(w, &solution);
        if matcher.matches(&f) {
            Some((*w, f))
        } else {
            None
        }
    });

    let mut by_constraint: HashMap<Constraint, Vec<(Word, Feedback)>> = HashMap::new();
    for (w, f) in selected {
        by_constraint
            .entry(Constraint::from_word_and_feedback(&w, &f))
            .or_insert(Vec::new())
            .push((w, f));
    }

    let mut constraint = &Constraint::empty();
    let empty = Vec::new();
    let mut res = by_constraint
        .get(&constraint)
        .unwrap_or_else(|| &empty)
        .iter()
        .take(k)
        .map(|x| *x)
        .collect::<Vec<_>>();

    while res.len() < k {
        let (nc, nv) = match next_group_of_guesses(&by_constraint, &constraint) {
            Some(x) => x,
            None => break,
        };
        constraint = nc;
        for item in nv.iter().take(k - res.len()) {
            res.push(*item);
        }
    }

    res
}
