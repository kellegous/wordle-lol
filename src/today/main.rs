use std::error::Error;

use chrono::Local;
use wordle_lol::{nytimes, Feedback};

fn main() -> Result<(), Box<dyn Error>> {
    let data = nytimes::Data::fetch()?;
    let solution = data.solution_on(Local::now().date().naive_local());

    for word in data.guesses() {
        let feedback = Feedback::from_word(word, &solution);
        println!("{} {}", word, feedback);
    }

    Ok(())
}
