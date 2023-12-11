use super::solution::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(?P<hand>[AKQJT2-9]+) (?P<bid>\d+)").unwrap();
}

fn get_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!(),
    }
}

fn get_value2(c: char) -> u32 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!(),
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    score: Option<u32>,
}

fn counts<I>(mut iter: I) -> HashMap<u32, usize>
where
    I: Iterator<Item = u32>,
{
    let mut counts = HashMap::new();
    while let Some(n) = iter.next() {
        if counts.contains_key(&n) {
            counts.get_mut(&n).map(|c| *c += 1);
        } else {
            counts.insert(n, 1);
        }
    }

    counts
}

impl Hand {
    fn new(data: String, val_func: fn(char) -> u32) -> Self {
        let parsed = LINE_RE.captures(&data).unwrap();

        let cards = parsed
            .name("hand")
            .unwrap()
            .as_str()
            .chars()
            .map(val_func)
            .collect();
        let bid = parsed.name("bid").unwrap().as_str().parse::<u32>().unwrap();

        Self {
            cards,
            bid,
            score: None,
        }
    }

    fn compute_score(&self) -> u32 {
        let card_counts = counts(self.cards.iter().cloned());
        let max_count = card_counts.values().max().unwrap();
        let len = card_counts.len();

        if *max_count == 5 {
            7
        } else if *max_count == 4 {
            6
        } else if *max_count == 3 && len == 2 {
            5
        } else if *max_count == 3 {
            4
        } else if *max_count == 2 && len == 3 {
            3
        } else if *max_count == 2 {
            2
        } else {
            1
        }
    }

    fn get_all_possible_hands(&self) -> Vec<Self> {
        let card_counts = counts(self.cards.iter().cloned());
        let has_j = card_counts.contains_key(&1);

        if !has_j {
            vec![self.clone()]
        } else {
            let mut hands = vec![];
            let j_idx = self.cards.iter().position(|c| *c == 1).unwrap();

            for i in 2..14 {
                let mut hand = self.clone();
                hand.cards[j_idx as usize] = i;
                hands.extend(hand.get_all_possible_hands());
                // eprintln!("self.bid: {}, {:?}", self.bid, hands);
            }

            hands
        }
    }

    fn compute_score2(&self) -> u32 {
        self.get_all_possible_hands()
            .iter()
            .map(|h| h.compute_score())
            .max()
            .unwrap()
    }

    fn compute_secondary_score(&self, other: &Hand) -> Ordering {
        for (i, card) in self.cards.iter().enumerate() {
            let other_card = other.cards[i];
            if *card > other_card {
                return Ordering::Greater;
            } else if other_card > *card {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }

    fn compare(&self, other: &Hand) -> Ordering {
        let self_score = self.compute_score();
        let other_score = other.compute_score();

        if self_score > other_score {
            Ordering::Greater
        } else if self_score < other_score {
            Ordering::Less
        } else {
            self.compute_secondary_score(other)
        }
    }

    fn compare2(&self, other: &Hand) -> Ordering {
        let self_score = self.score.unwrap();
        let other_score = other.score.unwrap();

        if self_score > other_score {
            Ordering::Greater
        } else if self_score < other_score {
            Ordering::Less
        } else {
            self.compute_secondary_score(other)
        }
    }
}

pub struct Day7;

impl Day7 {
    fn parse(path: &str, val_func: fn(char) -> u32) -> Result<Vec<Hand>> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut data = String::new();
        let mut hands = vec![];

        loop {
            let n = reader.read_line(&mut data)?;
            if n == 0 {
                break;
            }
            hands.push(Hand::new(data.clone(), val_func));
            data.clear();
        }
        Ok(hands)
    }
}

impl Solution for Day7 {
    fn problem1(path: &str) -> std::io::Result<()> {
        let mut hands = Day7::parse(path, get_value)?;
        hands.sort_by(|a, b| a.compare(b));
        let total = hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum::<u32>();

        println!("Got answer to day 7 problem 1: {}", total);

        Ok(())
    }

    fn problem2(path: &str) -> std::io::Result<()> {
        let mut hands = Day7::parse(path, get_value2)?;
        hands
            .iter_mut()
            .for_each(|hand| hand.score = Some(hand.compute_score2()));
        hands.sort_by(|a, b| a.compare2(b));
        let total = hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum::<u32>();

        println!("Got answer to day 7 problem 2: {}", total);

        Ok(())
    }
}
