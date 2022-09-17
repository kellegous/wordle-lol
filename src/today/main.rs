use std::error::Error;

use chrono::Local;
use wordle_lol::nytimes;

fn main() -> Result<(), Box<dyn Error>> {
    let data = nytimes::Data::fetch()?;
    let solution = data.solution_on(Local::now().date().naive_local());
    println!("{}", solution);
    Ok(())
}
