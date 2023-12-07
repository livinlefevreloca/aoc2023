use super::solution::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{BufReader, BufRead, Result};
use std::fs::File;
use std::ops::Range as StdRange;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(?P<dest>\d+) (?P<src>\d+) (?P<len>\d+)").unwrap();
    static ref HEADER_RE: Regex = Regex::new(r"(?P<name>[a-z\-]+) map:").unwrap();
    static ref SEED_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref SEED_RANGE_RE: Regex = Regex::new(r"(?P<start>\d+) (?P<len>\d+)").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn contains(&self, n: u64) -> bool {
        n >= self.start && n < self.end
    }

    fn has_overlap(&self, other: &Range) -> bool {
        self.contains(other.start) || self.contains(other.end-1) || other.contains(self.start) || other.contains(self.end-1)
    }

    fn offset(&self, offset: u64) -> u64 {
        self.start + offset
    }

    fn start(&self) -> u64 {
        self.start
    }

    fn rng(&self) -> StdRange<u64> {
        self.start..self.end
    }

    fn splice(&self, other: &Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        if other.start < self.start && other.end > self.end {
            (Some(Range::new(other.start, self.start)), Some(self.clone()), Some(Range::new(self.end, other.start)))
        } else if other.start < self.start && other.end < self.end && other.end > self.start {
            (Some(Range::new(other.start, self.start)), Some(Range::new(self.start, other.end)), None)
        }  else if other.start > self.start && other.end > self.end && other.end < self.start {
            (None, Some(Range::new(other.start, self.end)), Some(Range::new(self.end, other.end)))
        } else if other.start > self.start && other.end < self.end {
            (None, Some(self.clone()), None)
        }  else if other.start < self.start {
            (Some(other.clone()), None, None)
        } else if other.start > self.end {
            (None, None, Some(other.clone()))
        }
        else {
            eprintln!("{:?}, {:?}", &self, other);
            panic!()
        }
    }

}


#[derive(Debug)]
struct RangeMap {
    src: Range,
    dest: Range,
}

impl RangeMap {

    fn from_line(line: &str) -> Self {
        let capt = LINE_RE.captures(line).unwrap();
        let src_start = capt.name("src").unwrap().as_str().parse::<u64>().unwrap();
        let dest_start = capt.name("dest").unwrap().as_str().parse::<u64>().unwrap();
        let len = capt.name("len").unwrap().as_str().parse::<u64>().unwrap();


        let src = Range::new(src_start, src_start+len);
        let dest = Range::new(dest_start, dest_start+len);

        Self {
            src,
            dest,

        }
    }

    fn apply(&self, r: &Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        let (below, overlap, above) = self.src.splice(r);
        let applied = overlap.map(|rng| Range::new(self.get_dest(rng.start), self.get_dest(rng.end)));
        (below, applied, above)
    }

    fn dest_contains(&self, n: u64) -> bool {
        self.dest.contains(n)
    }

    fn source_contains(&self, n: u64) -> bool {
        self.src.contains(n)
    }

    fn get_dest(&self, n: u64) -> u64 {
        self.dest.offset(n - self.src.start())
    }

    fn has_overlap(&self, other: &Range) -> bool {
        self.src.has_overlap(other)
    }

    fn get_src(&self, n: u64) -> u64 {
        self.src.offset(n - self.dest.start())
    }
}

#[derive(Debug)]
struct MapList {
    ranges: Vec<RangeMap>,
}

impl MapList {

    fn new(lines: Vec<String>) -> Self {
        let ranges = lines.iter().map(|l| RangeMap::from_line(l)).collect();

        Self {
            ranges,
        }
    }

    fn get_dest(&self, n: u64) -> u64 {
        for range in &self.ranges {
            if range.source_contains(n) {
                return range.get_dest(n)
            }
        };

        n
    }

    fn get_src(&self, n: u64) -> u64 {
        for range in &self.ranges {
            if range.dest_contains(n) {
                return range.get_src(n)
            }
        };

        n

    }

    fn has_any_overlap(&self, range: &Range) -> bool {
        self.ranges.iter().any(|r| r.has_overlap(range))
    }

    fn apply(&self, mut ranges: Vec<Range>) -> Vec<Range> {
        let mut done = vec![];
        while let Some(r) = ranges.pop() {
            if !self.has_any_overlap(&r) {
                done.push(r);
                continue;
            }
            for rng in &self.ranges {
                let (below, applied, above) = rng.apply(&r);
                if let Some(appl) =  applied {
                    done.push(appl);
                    if let Some(bel) = below {
                        ranges.push(bel);
                    }

                    if let Some(ab) = above {
                        ranges.push(ab);
                    }
                }
            }
        }

        done
    }
}

pub struct Day5;

impl Day5 {

    fn parse_seeds_1(line: &str) -> Vec<Range> {
        SEED_RE
        .captures_iter(line)
        .map(
            |capt| {
                let start = capt.get(0).unwrap().as_str().parse::<u64>().unwrap();
                Range::new(start, start + 1)
            }
        ).collect()
    }

    fn parse_seeds_2(line: &str) -> Vec<Range> {
        SEED_RANGE_RE
        .captures_iter(line)
        .map(
            |capt| {
                let start = capt.name("start").unwrap().as_str().parse::<u64>().unwrap();
                let len = capt.name("len").unwrap().as_str().parse::<u64>().unwrap();
                Range::new(start, start + len)
            }
        ).collect()
    }

    fn parse_file(path: &str, parse_seeds_f: fn(&str) -> Vec<Range>) -> Result<(Vec<Range>, Vec<MapList>)> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        let mut seeds = Vec::new();
        let mut maps = Vec::new();
        loop {
            let n = reader.read_line(&mut line).unwrap();
            if n == 0 {
                break;
            }
            match &line.trim() {
                l if l.starts_with("seeds:") => {
                    seeds.extend(
                        parse_seeds_f(l)
                    );
                },
                l if HEADER_RE.is_match(l) => {
                    let mut map_lines = Vec::new();
                    let mut map_line = String::new();

                    reader.read_line(&mut map_line).unwrap();
                    while !map_line.trim().is_empty() {
                        map_lines.push(map_line.clone());
                        map_line.clear();
                        reader.read_line(&mut map_line).unwrap();
                    }

                    let map = MapList::new(map_lines);
                    maps.push(map);
                },
                l if l.trim().is_empty() => continue,
                _ => panic!(),
            }
            line.clear();

        }

        Ok((seeds, maps))
    }

    fn p2_brute(seeds: Vec<Range>, maps: Vec<MapList>) -> Option<u64> {

        let mut min = None;
        for i in 0.. {
            let mut res = i;
            for map in maps.iter().rev() {
                res = map.get_src(res);
            }

            for seed in &seeds {
                if seed.contains(res) {
                    min = Some(i);
                    break
                }
            }
            if min.is_some() {
                break;
            }
        }
        min
    }

    fn p2_new(seeds: Vec<Range>, maps: Vec<MapList>) -> Option<u64> {
       let mut mins = vec![];

        for seed in seeds {
            let mut ranges = vec![seed];
            for map in &maps {
                ranges = map.apply(ranges);
            }
            let min = ranges.iter().map(|r| r.start).min();
            mins.push(min.unwrap());
        }

        mins.into_iter().min()
    }

}

impl Solution for Day5 {

    fn problem1(path: &str) -> Result<()> {
        let (seeds, maps) = Day5::parse_file(path, Day5::parse_seeds_1)?;
        let mut results = vec![];

        for seed_range in &seeds {
            for seed in seed_range.rng() {
                let mut res = seed;
                for map in &maps {
                    res = map.get_dest(res);
                }
                results.push(res);
            }
        }

        println!("Got answer for Day 5 Problem 1: {}", results.iter().min().unwrap());


        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let (seeds, maps) = Day5::parse_file(path, Day5::parse_seeds_2)?;
        let min = Day5::p2_new(seeds, maps);
        println!("Got answer to Day 5 Problem 2: {}", min.unwrap());
        Ok(())
    }
}
