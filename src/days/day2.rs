use crate::days::solution::Solution;
use regex::Regex;
use std::io::Result;

pub(crate) struct Day2;

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
