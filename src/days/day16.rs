use super::solution::Solution;
use std::collections::HashSet;
use std::io::{prelude::*, BufReader, Result};
use std::fs::File;

const GRID_MAX_X: usize = 109;
const GRID_MAX_Y: usize = 109;

pub struct Day16;


#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {

    fn back_reflect(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::South => Direction::East,
        }
    }

    fn forward_reflect(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::South => Direction::West,
        }
    }

}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}


fn clamped_add(i: usize, j: usize, clamp: usize) -> usize {
    if clamp - j < i { clamp } else { i+j }
}

fn clamped_sub(i: usize, j: usize, clamp: usize) -> usize {
    if clamp + j > i { clamp } else { i - j }
}

impl Point {

    fn next(&self, direction: Direction) -> Option<Point> {
        match direction {
            Direction::North => {
                if clamped_sub(self.y, 1, 0) != self.y {
                    Some(Self { x: self.x, y: self.y - 1 })
                } else {
                    None
                }
            },
            Direction::South => {
                if clamped_add(self.y, 1, GRID_MAX_Y) != self.y {
                    Some(Self { x: self.x, y: self.y + 1 })
                } else {
                    None
                }
            },
            Direction::East => {
                if clamped_add(self.x, 1, GRID_MAX_X) != self.x {
                    Some(Self { x: self.x + 1, y: self.y })
                } else {
                    None
                }
            },
            Direction::West => {
                if clamped_sub(self.x, 1, 0) != self.x {
                    Some(Self { x: self.x - 1 , y: self.y })
                } else {
                    None
                }
            },
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Beam {
    loc: Point,
    direction: Direction
}

impl Beam {

    fn next(&self) -> Option<Point> {
        self.loc.next(self.direction)
    }

    fn interact(self, c: char) -> Vec<Self> {
        let mut new_beams = vec![];
        match c {
            '.' => {
                // eprintln!("Got '.' continuing");
                if let Some(loc) = self.loc.next(self.direction) {
                    new_beams.push(Self { loc, direction: self.direction })
                }
            },
            '\\' => {
                // eprintln!("Got '\\' reflector");
                if let Some(loc) = self.loc.next(self.direction.back_reflect()) {
                    new_beams.push(Self { loc, direction: self.direction.back_reflect() });
                }
            },

            '/' => {
                // eprintln!("Got '\\' reflector");
                if let Some(loc) = self.loc.next(self.direction.forward_reflect()) {
                    new_beams.push(Self { loc, direction: self.direction.forward_reflect() });
                }
            }

            '-' => {
                // eprintln!("Got '-' horizontal splitter");
                match &self.direction {
                    Direction::North | Direction::South => {
                        if let Some(loc) =  self.loc.next(Direction::East) {
                            new_beams.push(Self { loc, direction: Direction::East });
                        }
                        if let Some(loc) = self.loc.next(Direction::West) {
                            new_beams.push(Self { loc, direction: Direction::West });
                        }
                    },
                    Direction::East | Direction::West => {
                        if let Some(loc) = self.loc.next(self.direction) {
                            new_beams.push(Self { loc, direction: self.direction});
                        }
                    }
                }
            },
            '|' => {
                // eprintln!("Got '|' vertical splitter");
                match &self.direction {
                    Direction::East | Direction::West => {
                        if let Some(loc) = self.loc.next(Direction::North) {
                            new_beams.push(Self { loc , direction: Direction::North });
                        }
                        if let Some(loc) = self.loc.next(Direction::South) {
                            new_beams.push(Self { loc, direction: Direction::South });
                        }
                    },
                    Direction::North | Direction::South => {
                        if let Some(loc) = self.loc.next(self.direction) {
                            new_beams.push(Self { loc, direction: self.direction});
                        }
                    }
                }
            },
            a => {
                // eprintln!("Got invalid char: {}", a);
                panic!()
            }
        }

        new_beams
    }

}


struct Grid {
    beams: Vec<Beam>,
    grid: Vec<Vec<char>>,
    energized: HashSet<Point>,
}

impl Grid {

    fn new(grid: Vec<Vec<char>>, beam: Beam) -> Self {
        let mut energized = HashSet::new();
        energized.insert(beam.loc);
        Self {
            beams: vec![beam],
            grid,
            energized,
        }
    }

    fn find_candidate_starting_points(&self) -> Vec<Beam> {
        let mut candidates = vec![];
        for (x, direction) in [(0, Direction::East), (self.grid[0].len() - 1, Direction::West)] {
            for y in 0..self.grid.len() {
                let loc = Point { x, y };
                candidates.push(Beam { loc, direction })
            }
        }
        for (y, direction) in [(0, Direction::South), (self.grid.len() - 1,Direction::North) ] {
            for x in 0..self.grid.len() {
                let loc = Point { x, y };
                candidates.push(Beam { loc, direction })
            }
        }
        candidates
    }

    fn get_char_at_point(&self, p: Point) -> char {
        self.grid[p.y][p.x]
    }

    fn advance(&mut self) {
        let mut new_beams = vec![];
        for beam in &self.beams {
            new_beams.extend(beam.interact(self.get_char_at_point(beam.loc)))

        }
        for beam in &new_beams {
            self.energized.insert(beam.loc);
        }

        self.beams = new_beams;
    }

    fn run(&mut self) {
        let total = 700;
        for i in 0..total {
            // eprintln!("energized: {:?}", self.energized);
            self.advance();
            // eprintln!("New beams set to {:?}", self.beams);
        }

        for i in 0..GRID_MAX_X + 1 {
            eprintln!();
            for j in 0..GRID_MAX_Y + 1 {
                if self.energized.contains(&Point { x: j, y: i }) {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
        }
        eprintln!()
    }
}


impl Day16 {
    fn parse(path: &str) -> Result<Vec<Vec<char>>> {
        let file = File::open(path)?;
        Ok(BufReader::new(file).lines().map(|l| {
            l.unwrap().chars().collect::<Vec<char>>()
        }).collect())
    }

}


impl Solution for Day16 {

    fn problem1(path: &str) -> Result<()> {
        return Ok(());
        let parsed = Day16::parse(path)?;
        let mut grid = Grid::new(parsed, Beam { loc: Point { x: 0, y: 0 }, direction: Direction::East});
        grid.run();

        println!("Got answer to Day16 problem 1: {}", grid.energized.len());
        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let parsed = Day16::parse(path)?;
        let grid = Grid::new(parsed.clone(), Beam { loc: Point { x: 0, y: 0 }, direction: Direction::East });

        let mut results = vec![];
        let test_beams = grid.find_candidate_starting_points();
        eprintln!("Trying {} candidates", test_beams.len());
        for beam in test_beams {
            eprintln!("Trying beam: {:?}", beam);
            let mut test_grid = Grid::new(parsed.clone(), beam);
            test_grid.run();
            results.push(test_grid.energized.len());
        }


        println!("Got answer to Day16 problem 1: {}", results.iter().max().unwrap());
        Ok(())
    }
}
