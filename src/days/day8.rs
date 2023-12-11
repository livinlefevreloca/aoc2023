use super::solution::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Result},
};

lazy_static! {
    static ref NODE_RE: Regex =
        Regex::new(r"(?P<val>[A-Z0-9]+) = \((?P<left>[A-Z0-9]+), (?P<right>[A-Z0-9]+)\)").unwrap();
}

const TERM: &str = "ZZZ";

#[derive(Clone, Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Node {
    val: String,
    left: String,
    right: String,
}

fn parse_nodes(reader: &mut BufReader<File>) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();

    for line in reader.lines().skip(1) {
        let text = line.unwrap();
        let node_match = NODE_RE.captures(&text).unwrap();
        let val = node_match.name("val").unwrap().as_str().to_owned();
        let left = node_match.name("left").unwrap().as_str().to_owned();
        let right = node_match.name("right").unwrap().as_str().to_owned();

        let node = Node {
            val: val.clone(),
            left,
            right,
        };
        nodes.insert(val, node);
    }

    nodes
}

fn parse_instructions(reader: &mut BufReader<File>) -> Result<Vec<Instruction>> {
    let mut raw = String::new();
    reader.read_line(&mut raw)?;
    Ok(raw
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!(),
        })
        .collect())
}

fn gcd(mut i: u64, mut j: u64) -> u64 {
    while j != 0 {
        let tmp = j;
        j = i % j;
        i = tmp;
    }
    i
}

fn lcm(i: u64, j: u64) -> u64 {
    i * j / gcd(i, j)
}

fn lcms(n: Vec<u64>) -> u64 {
    n.into_iter().reduce(lcm).unwrap()
}

pub struct Day8;

impl Solution for Day8 {
    fn problem1(path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        // return Ok(());
        let instructions = parse_instructions(&mut reader)?;
        let nodes = parse_nodes(&mut reader);

        let mut current_node = nodes.get("AAA").unwrap();
        for (i, inst) in instructions.iter().cycle().enumerate() {
            if current_node.val == TERM {
                println!("Got answer for Day8 Problem1: {}", i);
                break;
            }

            current_node = match inst {
                Instruction::Left => nodes.get(&current_node.left).unwrap(),
                Instruction::Right => nodes.get(&current_node.right).unwrap(),
            }
        }

        Ok(())
    }

    fn problem2(path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let instructions = parse_instructions(&mut reader)?;
        let nodes = parse_nodes(&mut reader);

        let mut current_nodes: Vec<&Node> =
            nodes.values().filter(|n| n.val.ends_with('A')).collect();

        let mut multiples = vec![];

        for current_node in current_nodes.iter_mut() {
            for (i, inst) in instructions.iter().cycle().enumerate() {
                if current_node.val.ends_with('Z') {
                    multiples.push(i as u64);
                    break;
                }

                *current_node = match inst {
                    Instruction::Left => nodes.get(&current_node.left).unwrap(),
                    Instruction::Right => nodes.get(&current_node.right).unwrap(),
                }
            }
        }
        println!("Got answer for problem2 day 8, {}", lcms(multiples));

        Ok(())
    }
}
