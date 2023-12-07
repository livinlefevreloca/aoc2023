use days::{day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5, day6::Day6, solution::Solution};
use std::env::args;

mod days;

fn main() {
    let mut args_list = args();
    args_list.next();
    let problem = args_list.next();
    let file = args_list.next().expect("Missing file path");
    match problem.as_deref() {
        Some("1") => {
            Day1::problem1(&file).unwrap();
            Day1::problem2(&file).unwrap();
        }
        Some("2") => {
            Day2::problem1(&file).unwrap();
            Day2::problem2(&file).unwrap();
        }
        Some("3") => {
            Day3::problem1(&file).unwrap();
            Day3::problem2(&file).unwrap();
        }
        Some("4") => {
            Day4::problem1(&file).unwrap();
            Day4::problem2(&file).unwrap();
        },
        Some("5") => {
            Day5::problem1(&file).unwrap();
            Day5::problem2(&file).unwrap();
        }
        Some("6") => {
            Day6::problem1(&file).unwrap();
            Day6::problem2(&file).unwrap();
        }
        _ => eprint!("usage: cargo run -- <problem_number>"),
    }
}



