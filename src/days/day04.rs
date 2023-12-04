use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<Scratchcard>().unwrap())
        .into_iter()
        .map(|c| match c.winning() {
            0 => 0,
            i => 2u32.pow(i.saturating_sub(1)),
        })
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let cards: Vec<Scratchcard> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut counts = cards
        .clone()
        .into_iter()
        .map(|c| (c.id, 1))
        .collect::<HashMap<u32, u32>>();

    for c in &cards {
        let w = c.winning();
        let n = *counts.get(&c.id).unwrap();
        for i in 0..w {
            counts.entry(c.id + 1 + i).and_modify(|count| *count += n);
        }
    }

    counts.values().sum()
}

#[derive(Clone, Debug)]
struct Scratchcard {
    id: u32,
    winning: HashSet<u32>,
    has: HashSet<u32>,
}

impl Scratchcard {
    pub fn winning(&self) -> u32 {
        self.winning.intersection(&self.has).count() as u32
    }
}

impl FromStr for Scratchcard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = s.split_once(':').unwrap();
        let id = card.strip_prefix("Card").unwrap().trim().parse().unwrap();
        let (winning, has) = numbers.split_once('|').unwrap();
        let winning = winning
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let has = has
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Scratchcard { id, winning, has })
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/04.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(25571, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(8805731, super::two(&input));
    }
}
