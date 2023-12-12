use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::utils::grid::{Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> u64 {
    let mut grid = parse_grid(&input);
    expand(&mut grid, 2);
    grid.map
        .keys()
        .tuple_combinations()
        .map(|(&a, &b)| {
            let dx = a.x.abs_diff(b.x);
            let dy = a.y.abs_diff(b.y);
            let dist = dx + dy;
            dist as u64
        })
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u64 {
    let mut grid = parse_grid(&input);
    expand(&mut grid, 1_000_000);
    grid.map
        .keys()
        .tuple_combinations()
        .map(|(&a, &b)| {
            let dx = a.x.abs_diff(b.x);
            let dy = a.y.abs_diff(b.y);
            let dist = dx + dy;
            dist as u64
        })
        .sum()
}

fn expand(grid: &mut Grid<Tile>, factor: usize) {
    let mut empty_row_idxs = Vec::new();
    let mut empty_col_idxs = Vec::new();
    for i in 0..grid.height {
        if grid.row(i as isize).iter().count() == 0 {
            empty_row_idxs.push(i);
        }
    }
    for i in 0..grid.width {
        if grid.col(i as isize).iter().count() == 0 {
            empty_col_idxs.push(i);
        }
    }

    let mut new_points = HashMap::new();
    for p in grid.map.keys() {
        let empty_cols = empty_col_idxs
            .iter()
            .filter(|c| (**c as isize) < p.x)
            .count();
        let empty_rows = empty_row_idxs
            .iter()
            .filter(|c| (**c as isize) < p.y)
            .count();
        let shift_x = empty_cols * factor - empty_cols;
        let shift_y = empty_rows * factor - empty_rows;
        let p = *p
            + Point2 {
                x: shift_x as isize,
                y: shift_y as isize,
            };
        new_points.insert(p, Tile::Galaxy);
    }

    grid.map = new_points;
}

fn parse_grid(input: &str) -> Grid<Tile> {
    Grid::from_vec(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => None,
                        _ => Some(Tile::Galaxy),
                    })
                    .collect()
            })
            .collect(),
    )
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Galaxy,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#")
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/11.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(9648398, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(618800410814, super::two(&input));
    }
}
