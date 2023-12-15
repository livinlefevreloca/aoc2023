use super::solution::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::ascii::AsciiExt;
use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::thread::{spawn, sleep, self};
use std::time::Duration;
use std::sync::{mpsc::{Receiver, Sender, channel}, Arc, Mutex};
use std::io::{prelude::*, BufReader, Result};

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(?P<row>[\.#\?]+) (?P<numbers>[\d,]+)").unwrap();
    static ref GROUP_RE: Regex = Regex::new(r"#+").unwrap();
    static ref GROUP_AND: Regex = Regex::new(r"").unwrap();
}

enum Event {
    Work{
        row: String,
        groups: VecDeque<String>,
        spec: Vec<i32>,
        original: String,
    },
    Stop,
    Result{total: u64, row: String},
}

pub struct Day12;

impl Day12 {
    fn parse(path: &str) -> Result<Vec<(String, Vec<i32>)>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rows = vec![];
        for line in reader.lines() {
            let text = line?;
            let matches = LINE_RE.captures(&text).unwrap();
            let row = matches.name("row").unwrap().as_str().to_owned();
            let numbers: Vec<i32> = matches
                .name("numbers")
                .unwrap()
                .as_str()
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            rows.push((row, numbers));
        }
        Ok(rows)
    }

    fn parse_with_expansion(path: &str) -> Result<Vec<(String, String, Vec<i32>)>> {
        let data = Day12::parse(path)?;
        let mut expanded = vec![];

        for (row, spec) in data {
            expanded.push((
                row.clone(),
                (0..5)
                    .map(|_| row.clone())
                    .collect::<Vec<String>>()
                    .join("?"),
                spec.repeat(5),
            ));
        }

        Ok(expanded)
    }

    fn spec_to_groups(row: &str, spec: &[i32]) -> VecDeque<String> {
        let mut groups = VecDeque::new();
        let max = spec.len() - 1;
        let starting_dots = row.chars().take_while(|c| *c == '.').count();
        let ending_dots = row.chars().rev().take_while(|c| *c == '.').count();
        for (i, s) in spec.iter().enumerate() {
            match i {
                0  => groups.push_back([".".repeat(starting_dots), "#".repeat(*s as usize), ".".to_string()].join("")),
                l if l == max => groups.push_back([".", &"#".repeat(*s as usize), &".".repeat(ending_dots)].join("")),
                _ => groups.push_back([".", &"#".repeat(*s as usize), "."].join("")),
            }
        }

        groups
    }

    fn get_all_combinations_2(
        row: &[char],
        mut groups: VecDeque<String>,
        mut row_ptr: usize,
        original: &[i32],
    ) -> usize {
        // eprintln!("-------------------------");
        // eprintln!(
        //     "get_all_combinations called with: row: {:?} and ptr: {}, row_len: {}",
        //     String::from_iter(row),
        //     row_ptr,
        //     row.len()
        // );
        // eprintln!("-------------------------");
        if groups.is_empty() {
            let str_row = String::from_iter(row).replace('?', ".");
            if Day12::matches_row(&str_row, original) {
                // println!(
                //     "Got row: {:?}, row_ptr: {}, returning 1",
                //     String::from_iter(row),
                //     row_ptr
                // );
                return 1;
            } else {
                // eprintln!("got invalid row {} for spec: {:?}. returning 0", String::from_iter(row), original);
                return 0
            }
        }

        let mut total = 0;
        let group: Vec<char> = groups.pop_front().unwrap().chars().collect();

        loop {
            let mut altered_row = row.to_vec();
            let fill = altered_row[0..row_ptr]
                .iter()
                .collect::<String>()
                .replace('?', ".");
            altered_row.splice(0..row_ptr, fill.chars().collect::<Vec<char>>());
            while row_ptr < (row.len() - group.len() - 1)
                && !Day12::validate(&altered_row[..row_ptr + group.len()], &group, row_ptr)
            {
                if altered_row[row_ptr] == '?' {
                    altered_row[row_ptr] = '.';
                }
                row_ptr += 1;
            }

            if row_ptr > row.len() - group.len() {
                // eprintln!("Breaking early: row_ptr: {}, group_len: {}, row_len: {}", row_ptr, group.len(), row_ptr);
                break;
            }

            if Day12::validate(&altered_row[..row_ptr + group.len()], &group, row_ptr) {
                // eprintln!("Altering row: {:?}, range: {:?} with {:?}", String::from_iter(&altered_row), row_ptr..row_ptr + group.len(), String::from_iter(group.clone()));
                altered_row.splice(row_ptr..row_ptr + group.len(), group.clone());
                // eprintln!(
                //     "making recursive call with: {:?}, {:?}, {}",
                //     String::from_iter(&altered_row),
                //     groups.clone(),
                //     row_ptr + group.len() - 1
                // );
                total += Day12::get_all_combinations_2(
                    &altered_row,
                    groups.clone(),
                    row_ptr + group.len() - 1,
                    original,
                );
            }
            // eprintln!(
            //     "Returned from recursive call -> row: {:?}, row_ptr: {}, group: {:?}",
            //     String::from_iter(&altered_row),
            //     row_ptr,
            //     String::from_iter(&group)
            // );
            row_ptr += 1;
            if row_ptr > row.len() - group.len() {
                // eprintln!("Breaking at loop end: row_ptr: {}, group_len: {}, row_len: {}", row_ptr, group.len(), row.len());
                break;
            }
        }

        // eprintln!("returning total: {}", total);
        total
    }

    fn validate(row: &[char], group: &[char], mut ptr: usize) -> bool {
        if group.starts_with(&['#']) && row[..ptr].ends_with(&['#']) {
            return false;
        }
        let mut group_ptr = 0;
        while ptr < row.len() && (row[ptr] == group[group_ptr] || row[ptr] == '?') {
            ptr += 1;
            group_ptr += 1;
        }

        ptr == row.len()
    }

    fn get_all_combinations(row: &str, spec: &[i32]) -> usize {
        if !row.contains('?') {
            if Day12::matches_row(row, spec) {
                1
            } else {
                0
            }
        } else {
            let mut all_combinations = 0;
            for comb in ["#", "."].into_iter().map(|c| row.replacen('?', c, 1)) {
                all_combinations += Day12::get_all_combinations(&comb, spec);
            }

            all_combinations
        }
    }

    fn matches_row(row: &str, spec: &[i32]) -> bool {
        // eprintln!("Checking: {}", row);
        let groups = row
            .split(|c| c == '.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        if groups.len() != spec.len() {
            return false;
        }
        groups
            .iter()
            .zip(spec)
            .filter(|(g, s)| g.len() == **s as usize)
            .count()
            == spec.len()
    }

    fn parrallel_find_combinations(sender: Sender<Event>, reciever: Arc<Mutex<Receiver<Event>>>) {
        loop {
            let some_event = match reciever.try_lock().ok() {
                Some(rx) => {
                    rx.try_recv().ok()
                },
                _ => None
            };

            if let Some(Event::Work{row, groups, spec, original}) = some_event {
                let total = Day12::get_all_combinations_2(&row.chars().collect::<Vec<char>>(), groups, 0, &spec) as u64;
                sender.send(Event::Result{total, row: original}).unwrap();
            } else {
                sleep(Duration::from_millis(1));
            }
        }
    }
}

impl Solution for Day12 {
    fn problem1(path: &str) -> std::io::Result<()> {
        let rows = Day12::parse(path)?;
        let mut total = 0;
        for (row, spec) in rows {
            let groups = Day12::spec_to_groups(&row, &spec);
            // eprintln!("Checking groups: {:?} against: {:?}", groups, row);
            let row_total =
                Day12::get_all_combinations_2(&row.chars().collect::<Vec<char>>(), groups, 0, &spec);
            total += row_total;

        }
        println!("Got answer for Day12 problem1: {}", total);
        Ok(())
    }

    fn problem2(path: &str) -> std::io::Result<()> {
        let rows = Day12::parse_with_expansion(path)?;
        let workers = 11;
        let (wtx, mrx) =  channel();
        let (mtx, wrx) = channel();
        let cloneable_wrx = Arc::new(Mutex::new(wrx));

        if rows.len() == 1 {
            eprintln!("Processing single record");
            let (original, row, spec) = rows[0].clone();
            let groups = Day12::spec_to_groups(&row, &spec);
            let total = Day12::get_all_combinations_2(&row.chars().collect::<Vec<char>>(), groups, 0, &spec);
            eprintln!("Got result for {}, {}", original, total);
            return Ok(());
        }

        let mut handles = vec![];

        for _ in 0..workers {
            let cloned_wtx = wtx.clone();
            let cloned_wrx = cloneable_wrx.clone();
            let handle = thread::spawn( move|| {
                Day12::parrallel_find_combinations(cloned_wtx, cloned_wrx);
            });
            handles.push(handle);
        }

        let mut full_total = 0;
        let mut sent = 0;
        let mut recved = 0;
        let row_count = rows.len();
        for (original, row, spec) in rows {
            if sent == workers {
                match mrx.recv() {
                    Ok(Event::Result { total, row }) => {
                        eprintln!("Got result for {}, {}", row, total);
                        recved += 1;
                        full_total += total;
                    }
                    _ => panic!(),
                }
                sent -= 1;

            }
            let groups = Day12::spec_to_groups(&row, &spec);
            mtx.send(Event::Work { row, groups, spec, original }).unwrap();
            sent += 1;
        }


        while recved < row_count {
            match mrx.recv() {
                Ok(Event::Result { total, row }) => {
                    eprintln!("Got result for {}: {}", row, total);
                    recved += 1;
                    full_total += total;
                }
                _ => panic!(),
            }

        }
        println!("Got answer for Day12 problem 2: {}", full_total);

        for _ in 0..workers {
            mtx.send(Event::Stop).unwrap();
        }

        Ok(())
    }
}
