use clap::Parser;
use num_format::{Locale, ToFormattedString};
use std::{collections::HashMap, error::Error};
use wordle_lol::{find_guesses, nytimes, print_solution, Directive, Match, Matcher};
fn report_stats(mut num_guesses: Vec<usize>) {
    if num_guesses.is_empty() {
        return;
    }

    num_guesses.sort();

    let mut histogram = HashMap::new();
    for n in num_guesses.iter() {
        *histogram.entry(*n).or_insert(0) += 1;
    }

    let n = num_guesses.len();
    let max = *num_guesses.last().unwrap();
    let min = *num_guesses.first().unwrap();
    let median = num_guesses[n / 2];
    let avg = num_guesses.iter().sum::<usize>() as f64 / n as f64;

    println!("Total:          {}", n.to_formatted_string(&Locale::en));
    println!("Min Guesses:    {}", min.to_formatted_string(&Locale::en));
    println!("Max Guesses:    {}", max.to_formatted_string(&Locale::en));
    println!(
        "Median Guesses: {}",
        median.to_formatted_string(&Locale::en)
    );
    println!("Avg Guesses:    {:0.3}", avg);

    println!();
    println!("Guess Histogram");
    /*
        let dw = 60.0 / *hist.values().max().unwrap() as f64;
    for i in 1..=max {
        let v = *hist.get(&i).unwrap_or(&0);
        let w = v as f64 * dw;
        let bar = std::iter::repeat("░").take(w as usize).collect::<String>();
        println!(
            "{:2}: ░{} {} ({:0.1}%)",
            i,
            bar,
            v.to_formatted_string(&Locale::en),
            100.0 * v as f64 / n as f64
        );
    }
     */
    let dw = 60.0 / *histogram.values().max().unwrap() as f64;
    for i in 1..=max {
        let v = *histogram.get(&i).unwrap_or(&0);
        let w = v as f64 * dw;
        let b = std::iter::repeat("░").take(w as usize).collect::<String>();
        println!(
            "{:2}: ░{} {} ({:0.1}%)",
            i,
            b,
            v.to_formatted_string(&Locale::en),
            100.0 * v as f64 / n as f64
        );
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

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
        if args.verbose {
            print_solution(i, solution, &selected);
            println!();
        }
    }

    report_stats(num_guesses);
    Ok(())
}
