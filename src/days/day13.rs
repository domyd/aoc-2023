use itertools::Itertools;

use crate::utils::grid::{Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let grids = parse(input);
    let mut mirrors = Vec::new();
    for g in grids {
        for r in 0..g.height - 1 {
            if compare_rows(&g, r, r + 1) {
                let mirror = Mirror::Horizontal(r, r + 1);
                if check_mirror(&g, &mirror) {
                    mirrors.push(mirror);
                }
            }
        }
        for c in 0..g.width - 1 {
            if compare_cols(&g, c, c + 1) {
                let mirror = Mirror::Vertical(c, c + 1);
                if check_mirror(&g, &mirror) {
                    mirrors.push(mirror);
                }
            }
        }
    }

    mirrors
        .iter()
        .map(|m| match m {
            Mirror::Horizontal(y0, _) => (y0 + 1) * 100,
            Mirror::Vertical(x0, _) => x0 + 1,
        })
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    unimplemented!()
}

#[derive(Debug, Clone, Copy)]
enum Mirror {
    Horizontal(usize, usize),
    Vertical(usize, usize),
}

fn check_mirror(grid: &Grid<char>, mirror: &Mirror) -> bool {
    match mirror {
        Mirror::Horizontal(y0, y1) => {
            let after = *y1..grid.height;
            let before = (0..=*y0).rev();
            before
                .zip(after)
                .map(|(y0, y1)| compare_rows(&grid, y0, y1))
                .all(|b| b)
        }
        Mirror::Vertical(x0, x1) => {
            let after = *x1..grid.width;
            let before = (0..=*x0).rev();
            before
                .zip(after)
                .map(|(x0, x1)| compare_cols(&grid, x0, x1))
                .all(|b| b)
        }
    }
}

fn compare_rows(grid: &Grid<char>, y0: usize, y1: usize) -> bool {
    let a = row_points(&grid, y0).iter().map(|p| p.x).collect_vec();
    let b = row_points(&grid, y1).iter().map(|p| p.x).collect_vec();
    a == b
}

fn compare_cols(grid: &Grid<char>, x0: usize, x1: usize) -> bool {
    let a = col_points(&grid, x0).iter().map(|p| p.y).collect_vec();
    let b = col_points(&grid, x1).iter().map(|p| p.y).collect_vec();
    a == b
}

fn row_points(grid: &Grid<char>, row: usize) -> Vec<Point2> {
    grid.row(row as isize).iter().map(|x| x.0).collect_vec()
}

fn col_points(grid: &Grid<char>, col: usize) -> Vec<Point2> {
    grid.col(col as isize).iter().map(|x| x.0).collect_vec()
}

fn parse(input: &str) -> Vec<Grid<char>> {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|input| {
            Grid::from_vec(
                input
                    .lines()
                    .map(|l| {
                        l.chars()
                            .map(|c| match c {
                                '.' => None,
                                c => Some(c),
                            })
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/13.txt";

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
