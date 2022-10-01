// fn solve_all(data: &Data) {
//     let matcher = Matcher::new(
//         Match::Is(Directive::Green),
//         Match::IsNot(Directive::Green),
//         Match::IsNot(Directive::Green),
//         Match::IsNot(Directive::Green),
//         Match::Is(Directive::Green),
//     );

//     let mut count = 0;
//     let mut sum = 0;
//     for solution in data.solutions() {
//         let selected = select(
//             data.all().filter_map(|w| {
//                 let f = Feedback::from_word(w, &solution);
//                 if matcher.matches(&f) {
//                     Some((*w, f))
//                 } else {
//                     None
//                 }
//             }),
//             5,
//         );

//         sum += selected.len();
//         count += 1;

//         println!("{}", solution);
//         for (w, f) in selected.iter() {
//             println!("{} {}", f, w)
//         }
//         println!(
//             "{} {}",
//             Feedback::from_word(&solution, &solution),
//             &solution,
//         );
//     }

//     println!("{}", sum as f64 / count as f64);
// }

fn main() {}
