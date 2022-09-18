use std::error::Error;

use chrono::Local;
use std::collections::BTreeSet;
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

fn score_of(fb: &Feedback) -> usize {
    WORD_SIZE - fb.iter().filter(|&d| *d == Directive::Black).count()
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

    let selected = data
        .all()
        .map(|w| {
            let f = Feedback::from_word(w, &solution);
            (score_of(&f), *w, f)
        })
        .filter(|(_, _, f)| matcher.matches(f))
        .collect::<BTreeSet<(usize, Word, Feedback)>>();

    let selected = selected
        .iter()
        .take(5)
        .map(|(_, w, f)| (*w, *f))
        .collect::<Vec<(Word, Feedback)>>();

    println!("Wordle {} {}/6*", num, selected.len() + 1);
    println!();
    for (_, f) in selected.iter() {
        println!("{}", f);
    }
    println!("{}", Feedback::from_word(&solution, &solution));

    println!();
    for (w, _) in selected.iter() {
        println!("{}", w)
    }
    println!("{}", &solution);

    Ok(())
}
