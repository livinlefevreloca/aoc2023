use super::solution::Solution;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

pub struct Day9;

impl Day9 {
    fn parse(path: &str) -> Result<Vec<Vec<i32>>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut sequences = vec![];

        for line in reader.lines() {
            let text = line?;
            let sequence: Vec<i32> = text
                .split_whitespace()
                .map(|n: &str| n.parse::<i32>().unwrap())
                .collect();
            sequences.push(sequence);
        }

        Ok(sequences)
    }

    fn extend_sequence(mut seq: Vec<i32>) -> Vec<i32> {
        if seq.iter().all(|n| *n == 0) {
            seq.push(0);
            seq
        } else {
            let diffs: Vec<i32> = seq.iter().map_windows(|[x, y]| **y - **x).collect();
            let diffs = Day9::extend_sequence(diffs);
            let last = seq.last().unwrap();
            seq.push(last + diffs.last().unwrap());
            seq
        }
    }

    fn prepend_sequence(mut seq: VecDeque<i32>) -> VecDeque<i32> {
        if seq.iter().all(|n| *n == 0) {
            seq.push_front(0);
            seq
        } else {
            let diffs: VecDeque<i32> = seq.iter().map_windows(|[x, y]| **y - **x).collect();
            let diffs = Day9::prepend_sequence(diffs);
            let last = seq.front().unwrap();
            seq.push_front(last - diffs.front().unwrap());
            seq
        }
    }
}

impl Solution for Day9 {
    fn problem1(path: &str) -> Result<()> {
        let sequences = Day9::parse(path)?;
        let total = sequences
            .into_iter()
            .map(Day9::extend_sequence)
            .map(|s: Vec<i32>| *s.last().unwrap())
            .sum::<i32>();
        println!("Got answer to Day 9 Problem 1: {}", total);
        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let sequences: Vec<VecDeque<i32>> = Day9::parse(path)?
            .into_iter()
            .map(VecDeque::from_iter)
            .collect();
        let total = sequences
            .into_iter()
            .map(Day9::prepend_sequence)
            .map(|s: VecDeque<i32>| *s.front().unwrap())
            .sum::<i32>();
        println!("Got answer to Day 9 Problem 2: {}", total);
        Ok(())
    }
}
