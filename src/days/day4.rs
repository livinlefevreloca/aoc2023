use crate::days::solution::Solution;
use std::collections::HashSet;

struct Card {
    matches: usize,
}


pub(crate) struct Day4;

impl Solution for Day4 {
    fn problem1(path: &str) -> std::io::Result<()> {
        let lines = Self::read_input_into_lines(path)?;

        let total = lines.iter().map(|line|{
            let (_, rest) = line.split_once(':').unwrap();
            let (winning, have) = rest.split_once('|').unwrap();
            let winning_nums: HashSet<u32> = winning.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
            let have_nums: HashSet<u32> = have.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();

            let winners = have_nums.intersection(&winning_nums).count() as u32;
            if winners == 0 {
                0
            } else {
                (2_u32).pow(winners - 1)
            }
        }).sum::<u32>();

        println!("Got answer for day 4 problem 1: {}", total);

        Ok(())
    }

    fn problem2(path: &str) -> std::io::Result<()> {
        let lines = Self::read_input_into_lines(path)?;
        let cards: Vec<Card> = lines.iter().map(|line| {
            let (_, rest) = line.split_once(':').unwrap();

            let (winning, have) = rest.split_once('|').unwrap();
            let winning_nums: HashSet<u32> = winning.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
            let have_nums: HashSet<u32> = have.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
            let matches = have_nums.intersection(&winning_nums).count();

            Card {
                matches,
            }

        }).collect();

        let mut card_counts: Vec<u32> = (0..cards.len()).map(|_| 1).collect();
        for (i, card) in cards.iter().enumerate() {
            let current_count = card_counts[i];

            for j in (i+1..i+1+card.matches) {
                card_counts[j] += current_count;
            }

            eprintln!("card.matches: {}, counts: {:?}", card.matches, card_counts);
        }

        let total: u32 = card_counts.iter().sum::<u32>();

        println!("Got answer for day 4 problem 2: {}", total);


        Ok(())
    }
}
