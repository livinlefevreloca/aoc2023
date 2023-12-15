use super::solution::Solution;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader, Result};
use std::fs::File;
use std::mem::swap;

pub struct Day14;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Day14 {
    fn parse(path: &str) -> Result<Vec<Vec<char>>> {
        Ok(
            BufReader::new(File::open(path)?).lines().map(|line| line.unwrap().chars().collect::<Vec<char>>()).collect()
        )
    }

    fn tilt_north(dish: &mut [Vec<char>]) {
        for i in 0..dish.len() {
            if i == 0 {
                continue;
            }
            for j in 0..dish[0].len() {
                match dish[i][j] {
                    rock @ 'O' => {
                        for k in (0..i).rev() {
                            match dish[k][j] {
                                '.' => {
                                    dish[k][j] = rock;
                                    dish[k+1][j] = '.'
                                },
                                _ => break,
                            }
                        }
                    },
                    _ => continue
                }
            }
        }

    }

    fn tilt_south(dish: &mut [Vec<char>]) {
        for i in (0..dish.len()).rev() {
            if i == dish.len() - 1 {
                continue;
            }
            for j in 0..dish[0].len() {
                match dish[i][j] {
                    rock @ 'O' => {
                        for k in i+1..dish.len() {
                            match dish[k][j] {
                                '.' => {
                                    dish[k][j] = rock;
                                    dish[k-1][j] = '.'
                                },
                                _ => break,
                            }
                        }
                    },
                    _ => continue
                }
            }
        }

    }

    fn tilt_west(dish: &mut [Vec<char>]) {
        for j in 0..dish[0].len() {
            if j == 0 {
                continue;
            }
            for i in 0..dish.len() {
                match dish[i][j] {
                    rock @ 'O' => {
                        for k in (0..j).rev() {
                            match dish[i][k] {
                                '.' => {
                                    dish[i][k] = rock;
                                    dish[i][k+1] = '.'
                                },
                                _ => break,
                            }
                        }
                    },
                    _ => continue
                }
            }
        }
    }

    fn tilt_east(dish: &mut [Vec<char>]) {
        for j in (0..dish[0].len()).rev() {
            if j == dish[0].len() - 1 {
                continue;
            }
            for i in 0..dish.len() {
                match dish[i][j] {
                    rock @ 'O' => {
                        for k in j+1..dish[0].len() {
                            match dish[i][k] {
                                '.' => {
                                    dish[i][k] = rock;
                                    dish[i][k-1] = '.'
                                },
                                _ => break,
                            }
                        }
                    },
                    _ => continue
                }
            }
        }

    }

    fn tilt(dish: &mut [Vec<char>], direction: Direction) {
        match direction {
            Direction::North => Day14::tilt_north(dish),
            Direction::West => Day14::tilt_west(dish),
            Direction::South => Day14::tilt_south(dish),
            Direction::East => Day14::tilt_east(dish),
        }
    }

    fn cyle(dish: &mut [Vec<char>]) {
        [Direction::North, Direction::West, Direction::South, Direction::East].iter().map(|d| {
            Day14::tilt(dish, *d);
        }).for_each(drop);
    }

    fn count_load(dish: &[Vec<char>]) -> usize {
        let mut total = 0;
        let mut multiplier = dish.len();

        for row in dish {
            total += row.iter().filter(|c| **c == 'O').count() * multiplier;
            multiplier -= 1;
        }

        total
    }

}


impl Solution for Day14 {

    fn problem1(path: &str) -> Result<()> {
        let mut dish = Day14::parse(path)?;
        Day14::tilt(&mut dish, Direction::North);
        println!("Got answer for Day14 problem 1: {}", Day14::count_load(&dish));
        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let mut dish = Day14::parse(path)?;
        let mut seen: HashMap<String, (usize, usize)> = HashMap::new();
        let mut cycle = 0;
        let mut original = dish.clone();

        loop {
            Day14::cyle(&mut dish);
            let current =  dish.iter().cloned().map(String::from_iter).collect::<Vec<String>>().join("\n");
            if !seen.contains_key(&current) {
                seen.insert(current.clone(), (1, cycle));
            } else {
                let data = seen.get_mut(&current).unwrap();
                data.0 += 1;
                // if we've seen the configuration 4 times before than we can be very sure this si
                // a cycle
                if data.0 == 4 {
                    break
                }
            }
            cycle += 1;
        }

        let start = seen.values().filter(|d| d.0 > 2).map(|d|d.1).min().unwrap();
        let end = seen.values().filter(|d| d.0 > 2).map(|d|d.1).max().unwrap();
        let modu = end - start + 1;
        let cycles = ((1000000000 - start) % modu) + start;

        for _ in 0..cycles  {
            Day14::cyle(&mut original);
        }

        println!("Got answer for Day14 problem 2: {}", Day14::count_load(&original));
        Ok(())
    }
}


