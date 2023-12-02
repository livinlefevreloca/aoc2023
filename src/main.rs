use regex::Regex;
use solution::Solution;
use std::env::args;
use std::io::Result;

mod solution;

fn main() {
    let mut args_list = args();
    args_list.next();
    let problem = args_list.next();
    let file = args_list.next().expect("Missing file path");
    eprintln!("args: {:?}, {:?}", problem, file);
    match problem.as_deref() {
        Some("1") => {
            Day1::problem1(&file).unwrap();
            Day1::problem2(&file).unwrap();
        }
        Some("2") => {
            Day2::problem1(&file).unwrap();
            Day2::problem2(&file).unwrap();
        }
        _ => eprint!("usage: cargo run -- <problem_number>"),
    }
}

struct Day1;

impl Day1 {
    fn word_to_digit(word: String) -> String {
        // map a written number or an ascii digit to the
        // corresponding ascii digit. Panic if anything else
        // is passed
        let digit = match word.as_str() {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            a if a.chars().nth(0).unwrap().is_ascii_digit() => a,
            _ => panic!(),
        };
        digit.to_owned()
    }
}

impl solution::Solution for Day1 {
    fn problem1(path: &str) -> Result<()> {
        let lines = Day1::read_input_into_lines(path)?;

        let total = lines
            .iter()
            .map(|s| {
                let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
                String::from_iter([digits[0], digits[digits.len() - 1]])
                    .parse::<u32>()
                    .unwrap()
            })
            .sum::<u32>();

        println!("Got answer to problem1 part 1: {}", total);
        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let lines = Day1::read_input_into_lines(path)?;
        // Rust regex doesnt allow for overlapping matches. So in the case of `oneighthree` it
        // will only find `one` if capture_iters is called. Since we only need the first and
        // the last match though we can just reverse the line and the use a reversed regex to
        // find the digit or written digit. You could also use the same regex and the walk
        // backward in the string applying it to each substring but this is more work
        let digits_regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let rev_digits_regex =
            Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
        let total = lines
            .iter()
            .enumerate()
            .map(move |(_, s)| {
                let first = Day1::word_to_digit(digits_regex.find(s).unwrap().as_str().to_owned());
                let last = Day1::word_to_digit(
                    rev_digits_regex
                        .find(&s.to_owned().chars().rev().collect::<String>())
                        .unwrap()
                        .as_str()
                        .chars()
                        .rev()
                        .collect::<String>(),
                );
                let num = String::from_iter([first, last]);
                num.parse::<u32>().unwrap()
            })
            .sum::<u32>();

        println!("Got answer for problem1 part2: {}", total);

        Ok(())
    }
}

struct Day2;

impl Solution for Day2 {
    fn problem1(path: &str) -> Result<()> {
        let lines = Day2::read_input_into_lines(path).unwrap();
        let game_regex = Regex::new(r"Game (?P<game_id>\d+)").unwrap();
        let cubes_regex =
            Regex::new(r"(?P<cube_count>\d+) (?P<cube_color>red|blue|green)").unwrap();

        let total = lines
            .iter()
            .map(|l| {
                let game_id = game_regex
                    .captures(l)
                    .unwrap()
                    .name("game_id")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let not_possible = cubes_regex
                    .captures_iter(l)
                    .map(|c| {
                        let color = c.name("cube_color").unwrap().as_str();
                        let count: u32 = c
                            .name("cube_count")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();
                        match color {
                            "red" if count > 12 => true,
                            "green" if count > 13 => true,
                            "blue" if count > 14 => true,
                            _ => false,
                        }
                    })
                    .any(|b| b);
                (game_id, !not_possible)
            })
            .filter(|(_, possible)| *possible)
            .map(|(g, _)| g)
            .sum::<u32>();

        println!("Got answer for problem1 part1: {}", total);

        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let lines = Day2::read_input_into_lines(path).unwrap();
        let cubes_regex =
            Regex::new(r"(?P<cube_count>\d+) (?P<cube_color>red|blue|green)").unwrap();

        let total = lines
            .iter()
            .map(|l| {
                let cube_samples = cubes_regex.captures_iter(l);
                let minimums: &mut [u32] = &mut [0, 0, 0];
                for sample in cube_samples {
                    let color = sample.name("cube_color").unwrap().as_str();
                    let count: u32 = sample
                        .name("cube_count")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap();
                    match color {
                        "red" if count > minimums[0] => minimums[0] = count,
                        "green" if count > minimums[1] => minimums[1] = count,
                        "blue" if count > minimums[2] => minimums[2] = count,
                        _ => {}
                    }
                }
                minimums.iter().copied().reduce(|acc, n| n * acc).unwrap()
            })
            .sum::<u32>();

        println!("Got answer for problem2 part2: {}", total);

        Ok(())
    }
}
