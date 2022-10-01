use std::error::Error;
use wordle_lol::{find_guesses, nytimes, print_solution, Directive, Match, Matcher};

fn main() -> Result<(), Box<dyn Error>> {
    let data = nytimes::Data::fetch()?;

    let matcher = Matcher::new(
        Match::Is(Directive::Green),
        Match::IsNot(Directive::Green),
        Match::IsNot(Directive::Green),
        Match::IsNot(Directive::Green),
        Match::Is(Directive::Green),
    );

    let mut num_guesses = Vec::new();
    for (i, solution) in data.solutions().iter().enumerate() {
        let selected = find_guesses(data.all(), solution, &matcher, 5);
        num_guesses.push(selected.len() + 1);
        print_solution(i, solution, &selected);
        println!();
    }

    Ok(())
}
