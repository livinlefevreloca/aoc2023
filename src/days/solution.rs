use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub(crate) trait Solution {
    fn read_input_into_lines(path: &str) -> Result<Vec<String>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(n) if n > 0 => lines.push(line),
                Err(e) => return Result::Err(e),
                _ => break,
            };
        }

        Ok(lines)
    }

    fn read_input_into_grid(path: &str) -> Result<Vec<Vec<char>>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut grid = Vec::new();

        let mut buf = String::new();
        while let Ok(n) = reader.read_line(&mut buf) {
            if n == 0 {
                break;
            }

            grid.push(buf.trim().chars().collect());
            buf.clear();
        }

        Ok(grid)
    }

    fn problem1(path: &str) -> Result<()>;
    fn problem2(path: &str) -> Result<()>;
}
