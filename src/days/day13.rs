use super::solution::Solution;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, Result, BufReader};

pub struct Day13;

enum Type {
    Vert,
    Horiz,
}

impl Day13 {
    fn parse_vert(path: &str) -> Result<Vec<Vec<String>>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut all = vec![];
        let mut mirror = vec![];
        for line in reader.lines() {
            let text = line?;
            if text.chars().all(|c| c.is_whitespace()) && !mirror.is_empty() {
                if !mirror.is_empty() {
                    all.push(mirror.clone());
                    mirror.clear();
                }
                continue;
            }
            for (i, c) in text.chars().enumerate() {
                if mirror.len() == i {
                    mirror.push(String::from(c));
                } else {
                    mirror[i].push(c);
                }
            }
        }

        all.push(mirror);
        Ok(all)
    }

    fn flip_char(mut chars: Vec<String>, i: usize, j: usize) -> Vec<String> {
        let new_char = match chars.get(i).unwrap().chars().nth(j) {
            Some('.') => {
               '#'
            }
            Some('#') => {
                '.'
            }
            c => {
                eprintln!("Got bad char: {:?}", c);
                unreachable!()
            }
        };
        let s = chars[i].clone();
        let new_s: String = s.chars().enumerate().map(|(i,c)| if i == j { new_char } else { c }).collect();
        chars[i] = new_s;
        chars
    }

    fn parse_horizontal(path: &str) -> Result<Vec<Vec<String>>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut all = vec![];
        let mut mirror = vec![];
        for line in reader.lines() {
            let text = line?;
            if text.chars().all(|c| c.is_whitespace()) {
                if !mirror.is_empty() {
                    all.push(mirror.clone());
                    mirror.clear();
                }
                continue;
            }
            mirror.push(text);
        }
        all.push(mirror);
        Ok(all)
    }

    fn find_sym(mirror: &[String], reject: usize) -> usize {
        let mut ptr = 1;
        while ptr < mirror.len() {
            let mut left = ptr -1;
            let mut right =  ptr;
            let mut found_sym = false;
            loop {
                if mirror[left] == mirror[right] {
                    found_sym = true;
                } else {
                    if found_sym {
                        left += 1;
                        right -= 1;
                    }
                    break;
                }
                if left > 0 && right < mirror.len() - 1 {
                   left -= 1;
                   right += 1;
                } else {
                    break
                }
            }

            if found_sym && (left == 0 || right == mirror.len() - 1) && ptr != reject  {
                return ptr;
            } else {
                ptr += 1;
            }
        }

        0
    }
}


impl Solution for Day13 {
    fn problem1(path: &str) -> Result<()> {
        let horiz = Day13::parse_horizontal(path)?;
        let vert = Day13::parse_vert(path)?;

        let mut results: Vec<usize> = vec![];

        for (mirror_horiz, mirror_vert) in horiz.iter().zip(vert.iter()) {
            let horiz_line = Day13::find_sym(mirror_horiz, 0);
            if horiz_line == 0 {
                let vert_line = Day13::find_sym(mirror_vert, 0);

                if vert_line == 0 {
                    panic!("Failed to find any line");
                }

                results.push(vert_line);
            } else {
                results.push(horiz_line*100);
            }
        }

        let total = results.iter().sum::<usize>();
        println!("Got answer for day13 problem 1: {}", total);

        Ok(())
    }

    fn problem2(path: &str) -> Result<()> {
        let horiz = Day13::parse_horizontal(path)?;
        let vert = Day13::parse_vert(path)?;
        let mut results: HashMap<usize, (usize, Type)> = HashMap::new();
        for (k, (mirror_horiz, mirror_vert)) in horiz.iter().zip(vert.iter()).enumerate() {
            let horiz_line = Day13::find_sym(mirror_horiz, 0);
            if horiz_line == 0 {
                let vert_line = Day13::find_sym(mirror_vert, 0);

                if vert_line == 0 {
                    panic!("Failed to find any line");
                }

                results.insert(k, (vert_line, Type::Vert));
            } else {
                results.insert(k, (horiz_line, Type::Horiz));
            }
        }
        let mut new_results = vec![];
        for (k, (mirror_horiz, mirror_vert)) in horiz.iter().zip(vert.iter()).enumerate() {
            'outer: for i in 0..mirror_horiz.len() {
                for j in 0..mirror_vert.len() {
                    let flipped_horiz = Day13::flip_char(mirror_horiz.clone(), i, j);
                    let flipped_vert = Day13::flip_char(mirror_vert.clone(), j, i);
                    let (horiz_reject, vert_reject) = match results.get(&k) {
                       Some((n, Type::Vert)) => (0, *n),
                       Some((n, Type::Horiz)) => (*n, 0),
                       None => unreachable!(),
                    };

                    let horiz_line = Day13::find_sym(&flipped_horiz, horiz_reject);
                    if horiz_line == 0 {
                        let vert_line = Day13::find_sym(&flipped_vert, vert_reject);
                        if vert_line == 0 {
                            continue;
                        } else  {
                            new_results.push(vert_line);
                            break 'outer;
                        }

                    } else {
                        new_results.push(horiz_line*100);
                        break 'outer;

                    }
                }
            }

        }
        let total = new_results.iter().sum::<usize>();
        println!("Got answer for day13 problem 2: {}", total);


        Ok(())
    }
}
