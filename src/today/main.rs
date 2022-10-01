use chrono::Local;
use std::error::Error;
use wordle_lol::{find_guesses, nytimes, Directive, Feedback, Match, Matcher};

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

    let selected = find_guesses(data.all(), &solution, &matcher, 5);

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
