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

fn constraint_for(w: &Word, f: &Feedback) -> String {
    let mut s = BTreeSet::new();
    for (d, c) in f.iter().zip(w.iter()) {
        if *d == Directive::Yellow {
            s.insert(c.char());
        }
    }
    s.iter().collect::<String>()
}

fn select(it: impl Iterator<Item = (Word, Feedback)>, k: usize) -> Vec<(Word, Feedback)> {
    let mut by_constraint: HashMap<String, Vec<(Word, Feedback)>> = HashMap::new();
    for (w, f) in it {
        by_constraint
            .entry(constraint_for(&w, &f))
            .or_insert(Vec::new())
            .push((w, f));
    }

    let mut items = by_constraint.values().collect::<Vec<_>>();
    items.sort_by(|&a, &b| b.len().cmp(&a.len()));

    let mut res = Vec::new();
    for item in items {
        for (w, f) in item {
            res.push((*w, *f));
            if res.len() == k {
                return res;
            }
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
