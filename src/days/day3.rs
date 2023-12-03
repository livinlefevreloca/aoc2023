use lazy_static::lazy_static;
use regex::Regex;
use crate::days::solution::Solution;
use std::io::Result;

pub(crate) struct Day3;

lazy_static! {
    static ref SYMBOL_RE: Regex = Regex::new(r"[^\.\w\d]").unwrap();
    static ref NUMBER_RE: Regex = Regex::new(r"\d+").unwrap();
}

impl Day3 {
    fn clamped_sub(num: usize, sub: usize, clamp: usize) -> usize {
        if (clamp + sub) > num {
            clamp
        } else {
            num - sub
        }
    }

    fn clamped_add(num: usize, add: usize, clamp: usize) -> usize {
        if (clamp - add) < num {
            clamp
        } else {
            num + add
        }
    }

    fn check_surrounding(i: usize, j: usize, k: usize, grid: &[Vec<char>]) -> bool {
        let line_len = grid[i].len() - 1;
        let i_min = Day3::clamped_sub(i, 1, 0);
        let i_max = Day3::clamped_add(i, 2, grid.len());
        let min = Day3::clamped_sub(j, 1, 0);
        let max = Day3::clamped_add(k, 1, line_len);

        for line in &grid[i_min..i_max] {
            let to_check = String::from_iter(&line[min..max]);
            if SYMBOL_RE.is_match(&to_check) {
                return true;
            }
        }

        false
    }

    fn extract_gear_ratio(i: usize, j: usize, grid: &[Vec<char>]) -> Option<u32> {
        let i_min = Day3::clamped_sub(i, 1, 0);
        let i_max = Day3::clamped_add(i, 2, grid.len());
        let j_min = Day3::clamped_sub(j, 1, 0);
        let j_max = Day3::clamped_add(j, 2, grid[0].len());

        let mut nums = vec![];
        for line in &grid[i_min..i_max] {
            let s = String::from_iter(line);
            let num_captures = NUMBER_RE.captures_iter(&s);

            for num_capture in num_captures {
                if let Some(num_match) = num_capture.get(0) {
                    let start = num_match.start();
                    let end = num_match.end() - 1;
                    if (j_min..j_max).contains(&start) || (j_min..j_max).contains(&end) {
                        let num = num_match.as_str().parse::<u32>().unwrap();
                        nums.push(num)
                    }
                }
            }
        }

        match nums.len() {
            n if n < 2 => None,
            n if n > 2 => panic!("Found more than 2 adjacent numbers in gear ratio"),
            _ => Some(nums.into_iter().reduce(|acc, n| acc * n).unwrap()),
        }
    }
}

impl Solution for Day3 {
    fn problem1(path: &str) -> Result<()> {
        let grid = Day3::read_input_into_grid(path)?;

        let mut total: u32 = 0;
        for (i, line) in grid.iter().enumerate() {
            let line_str = String::from_iter(line);
            for capture in NUMBER_RE.captures_iter(&line_str) {
                let num_match = capture.get(0).unwrap();
                let include = Day3::check_surrounding(i, num_match.start(), num_match.end(), &grid);
                if include {
                    total += num_match.as_str().parse::<u32>().unwrap();
                }
            }
        }

        println!("Got solution to Day3 Problem1: {}", total);
        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let grid = Day3::read_input_into_grid(path)?;

        let mut total: u32 = 0;
        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '*' {
                    if let Some(ratio) = Day3::extract_gear_ratio(i, j, &grid) {
                        total += ratio;
                    }
                } else {
                    continue;
                }
            }
        }

        println!("Got solution to Day3 Problem2: {}", total);
        Ok(())
    }
}
