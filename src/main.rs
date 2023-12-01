use std::env::args;
use std::io::Result;
use solution::Solution;
use regex::Regex;

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
        },
        _ => eprint!("usage: cargo run -- <problem_number>")
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
            a if a.chars().nth(0).unwrap().is_ascii_digit()  => a,
            _ => panic!()
        };
        digit.to_owned()
    }
}

impl solution::Solution for Day1 {
    fn problem1(path: &str) -> Result<()> {
        let lines = Day1::read_input_into_lines(path)?;

        let total = lines.iter().map(|s|{
            let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
            String::from_iter([digits[0], digits[digits.len()-1]]).parse::<u32>().unwrap()
        }).sum::<u32>();

        println!("Got answer to problem1 part 1: {}", total);
        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let lines = Day1::read_input_into_lines(path)?;
        // Rust regex doesnt allow for overlapping matches. So in the case of `oneighthree` it
        // will on find `one` if capture_iters is called. Since we only need the first and
        // the last match though we can just reverse the line and the use a reversed regex to
        // find the digit or written digit. You could also use the same regex and the walk
        // backward in the string applying it to each substring but this is more work
        let digits_regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let rev_digits_regex = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
        let total = lines.iter().enumerate().map(move |(_, s)| {
            let first = Day1::word_to_digit(digits_regex.find(s).unwrap().as_str().to_owned());
            let last_word = rev_digits_regex.find(&s.to_owned().chars().rev().collect::<String>()).unwrap().as_str().chars().rev().collect::<String>();
            let last = Day1::word_to_digit(last_word);
            let num = String::from_iter([first, last]);
            num.parse::<u32>().unwrap()
        }).sum::<u32>();

        println!("Got answer for problem1 part2: {}", total);

        Ok(())
    }

}
