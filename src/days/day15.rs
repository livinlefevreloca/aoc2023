use super::solution::Solution;
use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::io::{prelude::*, Result, BufReader};


pub struct Day15;


impl Day15 {
    fn parse(path: &str) -> Result<Vec<Vec<char>>> {
        let file =  File::open(path)?;
        let mut data = String::new();
        BufReader::new(file).read_to_string(&mut data)?;
        Ok(data.split(',').map(|s| s.trim().chars().collect()).collect())
    }

    fn hash(chars: Vec<char>) -> u32 {
        let mut h = 0;

        for c in chars {
            h += c as u32;
            h *= 17;
            h %= 256;
        }

        h
    }
}

struct LenseBox {
    lenses: HashMap<String, (u32, u32)>,
    current: u32,
}

impl LenseBox {
    fn new() -> Self {
        Self {
            lenses: HashMap::new(),
            current: 0,
        }
    }

    fn insert(&mut self, label: &str, focal: u32) {

        if self.lenses.contains_key(label) {
            let pos = self.lenses.get(label).unwrap().0;
            self.lenses.insert(label.to_owned(), (pos, focal));
        } else {
            self.lenses.insert(label.to_owned(), (self.current, focal));
            self.current += 1;
        }

    }

    fn remove(&mut self, label: &str) {
        if let Some((pos, _)) = self.lenses.remove(label) {
            self.lenses.values_mut().filter(|(p, _)| *p > pos).for_each(|(p, _)| *p -= 1 );
            self.current -= 1;
        }
    }
}


impl Solution for Day15{

    fn problem1(path: &str) -> std::io::Result<()> {
        let total = Day15::parse(path)?.into_iter().map(Day15::hash).sum::<u32>();
        println!("Got answer for Day 15 Problem 1: {}", total);
        Ok(())
    }

    fn problem2(path: &str) -> std::io::Result<()> {
        let instructions = Day15::parse(path)?;

        let mut buckets: Vec<LenseBox> = Vec::new();

        for _ in 0..256 {
            buckets.push(LenseBox::new());
        }

        for inst in instructions {
            let mut str_inst = String::from_iter(&inst);
            if inst.contains(&'=') {
                let (label, focal_str) = str_inst.split_once('=').unwrap();
                let focal = focal_str.parse::<u32>().unwrap();
                let hash_ = Day15::hash(label.chars().collect()) as usize;
                buckets.get_mut(hash_).unwrap().insert(label, focal);
            } else if  inst.contains(&'-') {
                str_inst.remove(str_inst.len() - 1);
                let hash_ = Day15::hash(str_inst.chars().collect()) as usize;
                buckets.get_mut(hash_).unwrap().remove(&str_inst);
            }

        }

        let total = buckets.iter().enumerate().map(|(i, b)| {
            b.lenses.values().map(|(pos, focal)| (1 + i as u32) * (*pos + 1) * *focal).sum::<u32>()
        }).sum::<u32>();

        println!("Got solution for Day15 Problem2: {}", total);

        Ok(())

    }

}
