#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(slice_split_once)]
use days::{
    day1::Day1, day10::Day10, day11::Day11, day12::Day12, day13::Day13, day14::Day14, day15::Day15, day2::Day2, day3::Day3, day4::Day4,
    day5::Day5, day6::Day6, day7::Day7, day8::Day8, day9::Day9, solution::Solution,
};
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
        }
        Some("5") => {
            Day5::problem1(&file).unwrap();
            Day5::problem2(&file).unwrap();
        }
        Some("6") => {
            Day6::problem1(&file).unwrap();
            Day6::problem2(&file).unwrap();
        }
        Some("7") => {
            Day7::problem1(&file).unwrap();
            Day7::problem2(&file).unwrap();
        }
        Some("8") => {
            Day8::problem1(&file).unwrap();
            Day8::problem2(&file).unwrap();
        }
        Some("9") => {
            Day9::problem1(&file).unwrap();
            Day9::problem2(&file).unwrap();
        }
        Some("10") => {
            Day10::problem1(&file).unwrap();
            Day10::problem2(&file).unwrap();
        }
        Some("11") => {
            Day11::problem1(&file).unwrap();
            Day11::problem2(&file).unwrap();
        }
        Some("12") => {
            Day12::problem1(&file).unwrap();
            Day12::problem2(&file).unwrap();
        }
        Some("13") => {
            Day13::problem1(&file).unwrap();
            Day13::problem2(&file).unwrap();
        }
        Some("14") => {
            Day14::problem1(&file).unwrap();
            Day14::problem2(&file).unwrap();
        },
        Some("15") => {
            Day15::problem1(&file).unwrap();
            Day15::problem2(&file).unwrap();
        }
        _ => eprint!("usage: cargo run -- <problem_number>"),
    }
}
