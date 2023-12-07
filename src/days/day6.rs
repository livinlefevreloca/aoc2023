use super::solution::Solution;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use regex::Regex;

pub struct Day6;

struct Race {
    time: u64,
    distance: u64
}

impl Race {

    fn calc_wins(&self) -> usize {
        (1..self.time).filter(|i| {
           (self.time - i) * i > self.distance
        }).count()

    }

}

impl Day6 {
    fn parse_races(path: &str) -> Result<Vec<Race>> {
        let f = File::open(path)?;
        let num_re = Regex::new(r"\d+").unwrap();
        let mut reader = BufReader::new(f);
        let mut time = String::new();
        let mut distance = String::new();

        reader.read_line(&mut time)?;
        reader.read_line(&mut distance)?;

        let races = num_re.captures_iter(&time).map(|c| c.get(0).unwrap().as_str().parse::<u64>().unwrap()).zip(
            num_re.captures_iter(&distance).map(|c| c.get(0).unwrap().as_str().parse::<u64>().unwrap())
        ).map(|(time, distance)| Race { time, distance }).collect();

        Ok(races)
    }
    fn parse_race(path: &str) -> Result<Race> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut time_parts = String::new();
        let mut distance_parts = String::new();

        reader.read_line(&mut time_parts)?;
        reader.read_line(&mut distance_parts)?;

        let time_str = time_parts.chars().filter(|c| !c.is_whitespace()).collect::<String>().replace("Time:", "");
        let distance_str = distance_parts.chars().filter(|c| !c.is_whitespace()).collect::<String>().replace("Distance:","");
        let time = time_str.parse::<u64>().unwrap();
        let distance  = distance_str.parse::<u64>().unwrap();


        let race = Race { time, distance };

        Ok(race)
    }
}

impl Solution for Day6 {
    fn problem1(path: &str) -> std::io::Result<()> {
        let races  = Day6::parse_races(path)?;
        let res = races.iter().fold(1, |acc, r| r.calc_wins() * acc);
        println!("Got solution for Day 6 problem 1: {}", res);
        Ok(())
    }

    fn problem2(path: &str) -> std::io::Result<()> {
        let race  = Day6::parse_race(path)?;
        let wins = race.calc_wins();
        println!("Got solution for Day 6 problem 2: {}", wins);

        Ok(())
    }
}
