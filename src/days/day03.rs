use core::num;
use std::path::Display;

use itertools::Itertools;

use crate::utils::grid::{Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let grid = parse_grid(input);
    // eprintln!("{}", &grid);

    let mut numbers: Vec<Number> = Vec::new();
    for (is_number, group) in &input
        .replace('\r', "")
        .replace('\n', "")
        .char_indices()
        .group_by(|(_, c)| c.is_digit(10))
    {
        if !is_number {
            continue;
        }

        let digits = group.collect::<Vec<_>>();
        let number =
            u32::from_str_radix(digits.iter().map(|x| x.1).collect::<String>().as_str(), 10)
                .unwrap();
        let idx = digits.first().unwrap().0;
        numbers.push(Number::new(number, idx));
    }

    let mut sum = 0u32;
    for n in numbers {
        let is_part = n
            .points_around(&grid)
            .into_iter()
            .any(|p| match grid.map.get(&p) {
                Some(Tile::Symbol(_)) => true,
                _ => false,
            });

        if is_part {
            sum += n.num;
        }
    }

    sum
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    unimplemented!()
}

#[derive(Clone, Copy, Debug)]
struct Number {
    num: u32,
    idx: usize,
    len: usize,
}

impl Number {
    pub fn new(num: u32, idx: usize) -> Self {
        Number {
            num,
            idx,
            len: num.to_string().len(),
        }
    }

    pub fn points_around(&self, grid: &Grid<Tile>) -> Vec<Point2> {
        fn point_from_index(index: usize) -> Point2 {
            let y = index / 10;
            let x = index % 10;
            let p = Point2 {
                x: x as isize,
                y: y as isize,
            };
            p
        }

        let mut v = Vec::new();
        let mut add_point = |i: isize| {
            if i >= 0 {
                // eprintln!("add_point {i}");
                v.push(point_from_index(i as usize));
            } else {
                // eprintln!("skip point {i}");
            }
        };

        // eprintln!("{:?}", &self);

        for i in -1..((self.len as isize) + 1) {
            let i = self.idx as isize + i;
            eprintln!("i = {i}");
            add_point(i - 10);
            if i < 0 || i == self.len as isize {
                add_point(i);
            }
            add_point(i + 10);
        }

        eprintln!("points around {}: {:?}", self.num, &v);
        v
    }
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Digit(u8),
    Symbol(char),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Digit(d) => {
                write!(f, "{}", d)?;
            }
            Tile::Symbol(c) => {
                write!(f, "{}", c)?;
            }
        };
        Ok(())
    }
}

fn parse_grid(s: &str) -> Grid<Tile> {
    let vec: Vec<Vec<Option<Tile>>> = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => None,
                    d @ '0'..='9' => Some(Tile::Digit(d.to_digit(10).unwrap() as u8)),
                    c => Some(Tile::Symbol(c)),
                })
                .collect()
        })
        .collect();

    Grid::from_vec(vec)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/03.txt";

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
