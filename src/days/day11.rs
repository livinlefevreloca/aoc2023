use super::solution::Solution;
use std::fs::File;
use std:: collections::HashSet;
use std::io::{prelude::*, BufReader, Result};

pub struct Day11;

impl Day11 {
    fn parse(path: &str) -> Result<Vec<Vec<char>>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut layers: Vec<Vec<char>> = vec![];

        for line in reader.lines() {
            let text = line?;
            layers.push(text.chars().collect());
        }

        Ok(layers)
    }

    fn find_galaxies(space: &[Vec<char>], expansion: usize) -> Vec<(i32, i32)> {
        let (empty_rows, empty_cols) = Day11::find_empty_space(space);

        let mut galaxies = vec![];
        for i in 0..space.len() {
            let i_plus = empty_rows.iter().filter(|row| **row < i).count() * expansion;
            for j in 0..space[0].len() {
                let j_plus = empty_cols.iter().filter(|col| **col < j).count() * expansion;
                if space[i][j] == '#' {
                    galaxies.push(((i + i_plus) as i32, (j + j_plus) as i32))
                }
            }
        }
        galaxies
    }

    fn find_empty_space(space: &[Vec<char>]) -> (Vec<usize>, Vec<usize>) {
        let mut rows = vec![];
        let mut columns = vec![];

        for (i, row) in space.iter().enumerate() {
            if row.iter().all(|c| *c != '#') {
                rows.push(i);
            }
        }

        for j in 0..space[0].len() {
            let col = space.iter().map(|row| row[j]).collect::<Vec<char>>();
            if col.iter().all(|c| *c != '#') {
                columns.push(j);
            }
        }


        (rows, columns)
    }
}



impl Solution for Day11 {
    fn problem1(path: &str) -> std::io::Result<()> {
        let space = Day11::parse(path)?;
        let galaxies = Day11::find_galaxies(&space, 1);

        let mut pairs = HashSet::new();
        for (i, g0) in galaxies.iter().enumerate() {
            for (j, g1) in galaxies.iter().enumerate() {
                if i != j && !pairs.contains(&(g1, g0)) {
                    pairs.insert((g0, g1));
                }
            }
        }

        let mut total = 0;
        for (g0, g1) in pairs.iter() {
            let diff = g1.0.abs_diff(g0.0) + g1.1.abs_diff(g0.1);
            total += diff;
        }

        println!("Got result for Problem 1 day 11: {}", total);

        Ok(())
    }

    fn problem2(path: &str) -> std::io::Result<()> {
        let space = Day11::parse(path)?;
        let galaxies = Day11::find_galaxies(&space, 999999);

        let mut pairs = HashSet::new();
        for (i, g0) in galaxies.iter().enumerate() {
            for (j, g1) in galaxies.iter().enumerate() {
                if i != j && !pairs.contains(&(g1, g0)) {
                    pairs.insert((g0, g1));
                }
            }
        }

        let mut total: u64 = 0;
        for (g0, g1) in pairs.iter() {
            let diff = g1.0.abs_diff(g0.0) + g1.1.abs_diff(g0.1);
            total += diff as u64;
        }
        println!("Got result for Problem 2 day 11: {}", total);

        Ok(())
    }
}
