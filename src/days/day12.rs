use std::str::FromStr;

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let combs = (1..4).cartesian_product(vec![true, false]).collect_vec();
    dbg!(&combs);

    // let mut records: Vec<Record> = input.lines().map(|l| l.parse().unwrap()).collect();
    // for r in &mut records {
    //     let mut arrangements = 0;
    //     loop {
    //         r.line.iter().com
    //     }
    // }
    todo!()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    unimplemented!()
}

#[derive(Debug, Clone)]
struct Record {
    line: Vec<Spring>,
    groups: Vec<u32>,
}

impl Record {
    pub fn verify(&self, springs: &[Spring]) -> bool {
        let groups = springs
            .iter()
            .group_by(|s| **s)
            .into_iter()
            .filter_map(|(k, v)| match k {
                Spring::Damaged => Some(v.count() as u32),
                _ => None,
            })
            .collect_vec();
        dbg!(&groups);
        self.groups == groups
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line, groups) = s.split_once(' ').unwrap();
        dbg!(&line, &groups);
        let line = line
            .chars()
            .map(|c| match c {
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                '?' => Spring::Unknown,
                _ => panic!("unknown spring"),
            })
            .collect();
        let groups = groups.chars().filter_map(|c| c.to_digit(10)).collect();
        Ok(Record { line, groups })
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/12.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(0, super::one(&input));
    }

    #[test]
    #[ignore]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(0, super::two(&input));
    }
}
