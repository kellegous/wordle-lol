use chrono::Local;
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use wordle_lol::{nytimes, Directive, Feedback, Word, WORD_SIZE};

#[derive(Copy, Clone)]
enum Match {
    Is(Directive),
    IsNot(Directive),
}

impl Match {
    fn matches(&self, d: Directive) -> bool {
        match self {
            Self::Is(x) => d == *x,
            Self::IsNot(x) => d != *x,
        }
    }
}

struct Matcher {
    matches: [Match; WORD_SIZE],
}

impl Matcher {
    fn new(a: Match, b: Match, c: Match, d: Match, e: Match) -> Matcher {
        Matcher {
            matches: [a, b, c, d, e],
        }
    }

    fn matches(&self, feedback: &Feedback) -> bool {
        self.matches
            .iter()
            .zip(feedback.iter())
            .all(|(m, d)| m.matches(*d))
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct Constraint {
    s: String,
}

impl Constraint {
    fn from_word_and_feedback(w: &Word, f: &Feedback) -> Constraint {
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

    fn is_compatible(&self, c: &Constraint) -> bool {
        c.len() > self.len() && self.s.chars().all(|v| c.s.contains(v))
    }

    fn len(&self) -> usize {
        self.s.len()
    }

    fn empty() -> Constraint {
        Constraint { s: String::new() }
    }
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

fn next_group<'a>(
    m: &'a HashMap<Constraint, Vec<(Word, Feedback)>>,
    key: &Constraint,
) -> Option<(&'a Constraint, &'a Vec<(Word, Feedback)>)> {
    m.iter()
        .filter(|(k, _)| key.is_compatible(k))
        .max_by(|(_, a), (_, b)| b.len().cmp(&a.len()))
}

fn select(it: impl Iterator<Item = (Word, Feedback)>, k: usize) -> Vec<(Word, Feedback)> {
    let mut by_constraint: HashMap<Constraint, Vec<(Word, Feedback)>> = HashMap::new();
    for (w, f) in it {
        by_constraint
            .entry(Constraint::from_word_and_feedback(&w, &f))
            .or_insert(Vec::new())
            .push((w, f));
    }

    let mut c = &Constraint::empty();
    let empty = Vec::new();
    let mut res = by_constraint
        .get(&c)
        .unwrap_or_else(|| &empty)
        .iter()
        .take(k)
        .map(|x| *x)
        .collect::<Vec<_>>();

    while res.len() < k {
        let (nc, nv) = match next_group(&by_constraint, &c) {
            Some(x) => x,
            None => break,
        };
        c = nc;
        for item in nv.iter().take(k - res.len()) {
            res.push(*item);
        }
    }

    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = nytimes::Data::fetch()?;
    let (num, solution) = data.solution_on(Local::now().date().naive_local());

    let matcher = Matcher::new(
        Match::Is(Directive::Green),
        Match::IsNot(Directive::Green),
        Match::IsNot(Directive::Green),
        Match::IsNot(Directive::Green),
        Match::Is(Directive::Green),
    );

    let selected = select(
        data.all().filter_map(|w| {
            let f = Feedback::from_word(w, &solution);
            if matcher.matches(&f) {
                Some((*w, f))
            } else {
                None
            }
        }),
        5,
    );

    println!("Wordle {} {}/6*", num, selected.len() + 1);
    println!();
    for (_, f) in selected.iter() {
        println!("{}", f);
    }
    println!("{}", Feedback::from_word(&solution, &solution));

    println!();
    for (w, f) in selected.iter() {
        println!("{} {}", f, w)
    }
    println!(
        "{} {}",
        Feedback::from_word(&solution, &solution),
        &solution,
    );

    println!();
    println!("VERSION: {}", data.version());

    Ok(())
}
