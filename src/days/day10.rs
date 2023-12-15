use super::solution::Solution;
use colored::Colorize;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

pub struct Day10;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn get_perpindicular(direction: Direction) -> [Direction; 2] {
    match direction {
        Direction::North | Direction::South => [Direction::West, Direction::East],
        Direction::East | Direction::West => [Direction::North, Direction::South],
    }
}

fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn get_new_perpindicular(
    old_direction: Direction,
    new_direction: Direction,
    perpindicular: Direction,
) -> Direction {
    if new_direction == perpindicular {
        get_opposite_direction(old_direction)
    } else {
        old_direction
    }
}

fn clamped_sub(i: usize, j: usize, clamp: usize) -> usize {
    if (clamp + j) > i {
        clamp
    } else {
        i - j
    }
}

fn clamped_add(i: usize, j: usize, clamp: usize) -> usize {
    if (clamp - j) < i {
        clamp
    } else {
        i + j
    }
}

impl Location {
    fn new((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }

    fn north(&self) -> (Self, Direction) {
        (
            Self {
                x: self.x,
                y: clamped_sub(self.y, 1, 0),
            },
            Direction::North,
        )
    }

    fn south(&self, max: usize) -> (Self, Direction) {
        (
            Self {
                x: self.x,
                y: clamped_add(self.y, 1, max),
            },
            Direction::South,
        )
    }

    fn east(&self, max: usize) -> (Self, Direction) {
        (
            Self {
                x: clamped_add(self.x, 1, max),
                y: self.y,
            },
            Direction::East,
        )
    }

    fn west(&self) -> (Self, Direction) {
        (
            Self {
                x: clamped_sub(self.x, 1, 0),
                y: self.y,
            },
            Direction::West,
        )
    }
}

struct PipeGrid {
    grid: Vec<Vec<char>>,
    start: Location,
}

fn get_loop(grid: &PipeGrid, start: Location) -> Vec<(Location, Direction)> {
    let mut path: Vec<(Location, Direction)> = Vec::new();
    let possible_directions = grid.find_possible_directions(&start);
    let (mut current, mut direction) = possible_directions[0];
    path.push((current, direction));

    while current != start {
        let next = grid.get_next_direction(&current, direction);
        (current, direction) = next;
        path.push(next);
    }

    path
}

fn print_loop(grid: &PipeGrid, loop_path: &[(Location, Direction)], in_set: &HashSet<Location>) {
    let loop_set: HashSet<&Location> = HashSet::from_iter(loop_path.iter().map(|p| &p.0));
    for (y, row) in grid.grid.iter().enumerate() {
        println!();
        for (x, c) in row.iter().enumerate() {
            let loc = Location::new((x, y));
            let mut to_bold = String::new();
            to_bold.push(*c);
            if loop_set.contains(&loc) {
                print!("{}", to_bold.bold());
            } else if in_set.contains(&loc) {
                print!("{}", to_bold.bold().red())
            } else {
                print!("{}", to_bold);
            }
        }
    }
    println!()
}

fn get_loop_area(grid: PipeGrid, loop_path: Vec<(Location, Direction)>, begin: Location) -> usize {
    let mut path = VecDeque::from_iter(&loop_path);
    let loop_set = HashSet::from_iter(loop_path.iter().map(|p| p.0));
    let start = grid.find_possible_directions(&begin)[0];
    path.push_front(&start);
    let perpindiculars = get_perpindicular(start.1);
    let mut sets = vec![];
    for perp in perpindiculars {
        let mut perpindicular = perp;
        let mut in_set = HashSet::new();
        for (location, direction) in loop_path.iter() {
            let next_direction = grid.get_next_direction(location, *direction).1;
            add_to_inner(
                &mut in_set,
                &loop_set,
                *location,
                perpindicular,
                grid.max_x(),
                grid.max_y(),
            );
            if *direction != next_direction {
                perpindicular = get_new_perpindicular(*direction, next_direction, perpindicular);
                add_to_inner(
                    &mut in_set,
                    &loop_set,
                    *location,
                    perpindicular,
                    grid.max_x(),
                    grid.max_y(),
                );
            }
        }
        sets.push(in_set);
    }

    let in_set = if sets[0].len() > sets[1].len() {
        sets[1].clone()
    } else {
        sets[0].clone()
    };

    print_loop(&grid, &loop_path, &in_set);
    in_set.len()
}

fn add_to_inner(
    inner: &mut HashSet<Location>,
    loop_set: &HashSet<Location>,
    loc: Location,
    dir: Direction,
    max_x: usize,
    max_y: usize,
) {
    match dir {
        Direction::North => {
            let mut prev_loc = loc;
            let mut new_loc = prev_loc.north().0;
            while !loop_set.contains(&new_loc) && prev_loc != new_loc {
                inner.insert(new_loc);
                prev_loc = new_loc;
                new_loc = prev_loc.north().0;
            }
        }
        Direction::South => {
            let mut prev_loc = loc;
            let mut new_loc = prev_loc.south(max_y).0;
            while !loop_set.contains(&new_loc) && prev_loc != new_loc {
                inner.insert(new_loc);
                prev_loc = new_loc;
                new_loc = prev_loc.south(max_y).0
            }
        }
        Direction::West => {
            let mut prev_loc = loc;
            let mut new_loc = prev_loc.west().0;
            while !loop_set.contains(&new_loc) && prev_loc != new_loc {
                inner.insert(new_loc);
                prev_loc = new_loc;
                new_loc = prev_loc.west().0
            }
        }
        Direction::East => {
            let mut prev_loc = loc;
            let mut new_loc = loc.east(max_x).0;
            while !loop_set.contains(&new_loc) && prev_loc != new_loc {
                inner.insert(new_loc);
                prev_loc = new_loc;
                new_loc = prev_loc.east(max_x).0
            }
        }
    }
}

impl PipeGrid {
    fn parse(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut start: Option<(usize, usize)> = None;

        for (i, line) in reader.lines().enumerate() {
            if let Some(Some(j)) = line
                .as_ref()
                .ok()
                .map(|pipes| pipes.chars().position(|c| c == 'S'))
            {
                start.replace((j, i));
            }
            grid.push(line?.chars().collect());
        }

        Ok(Self {
            grid,
            start: Location::new(start.unwrap()),
        })
    }

    fn get_pipe(&self, loc: &Location) -> char {
        self.grid[loc.y][loc.x]
    }

    fn max_y(&self) -> usize {
        self.grid.len() - 1
    }

    fn max_x(&self) -> usize {
        self.grid[0].len() - 1
    }

    fn get_next_direction(
        &self,
        current: &Location,
        direction: Direction,
    ) -> (Location, Direction) {
        match (self.get_pipe(current), direction) {
            ('|', Direction::North) => current.north(),
            ('|', Direction::South) => current.south(self.max_y()),
            ('-', Direction::East) => current.east(self.max_x()),
            ('-', Direction::West) => current.west(),
            ('F', Direction::North) => current.east(self.max_x()),
            ('F', Direction::West) => current.south(self.max_y()),
            ('7', Direction::North) => current.west(),
            ('7', Direction::East) => current.south(self.max_y()),
            ('L', Direction::South) => current.east(self.max_x()),
            ('L', Direction::West) => current.north(),
            ('J', Direction::South) => current.west(),
            ('J', Direction::East) => current.north(),
            ('S', _) => (*current, direction),
            _ => panic!(),
        }
    }

    fn find_possible_directions(&self, current: &Location) -> Vec<(Location, Direction)> {
        let mut directions = vec![];

        let north = current.north();
        if north.0.y != current.y
            && self.is_valid_incoming_direction(self.get_pipe(&north.0), Direction::North)
        {
            directions.push(north);
        }
        let west = current.west();
        if west.0.x != current.x
            && self.is_valid_incoming_direction(self.get_pipe(&west.0), Direction::West)
        {
            directions.push(west);
        }
        let east = current.east(self.max_x());
        if east.0.x <= self.max_x()
            && self.is_valid_incoming_direction(self.get_pipe(&east.0), Direction::East)
        {
            directions.push(east);
        }
        let south = current.south(self.max_y());
        if south.0.y <= self.max_y()
            && self.is_valid_incoming_direction(self.get_pipe(&south.0), Direction::South)
        {
            directions.push(south);
        }

        directions
    }

    fn is_valid_incoming_direction(&self, c: char, direction: Direction) -> bool {
        match direction {
            Direction::North => ['7', '|', 'F'].contains(&c),
            Direction::South => ['L', '|', 'J'].contains(&c),
            Direction::East => ['J', '7', '-'].contains(&c),
            Direction::West => ['L', 'F', '-'].contains(&c),
        }
    }
}

impl Day10 {}

impl Solution for Day10 {
    fn problem1(path: &str) -> std::io::Result<()> {
        let grid = PipeGrid::parse(path)?;
        let start = grid.start;
        let loop_path = get_loop(&grid, start);
        let max = loop_path.len() / 2;

        println!("Found answer to Day10 Problem 1: {}", max);
        Ok(())
    }

    fn problem2(path: &str) -> std::io::Result<()> {
        let grid = PipeGrid::parse(path)?;
        let start = grid.start;
        let loop_path = get_loop(&grid, start);
        let area = get_loop_area(grid, loop_path, start);
        println!("Found answer to Day10 Problem 2: {}", area);
        Ok(())
    }
}
