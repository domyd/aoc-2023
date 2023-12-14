use std::fmt::Display;

use crate::utils::grid::{BoundingBox2, Direction, Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    count(&tilt(&parse(&input), Direction::North))
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let grid = parse(&input);

    let mut history: Vec<Grid<Tile>> = Vec::new();
    history.push(grid.clone());
    let mut prev = grid;
    for i in 1.. {
        let current = cycle(&prev);

        if let Some(cycle_len) = history
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, r)| (r.map == current.map).then_some(i + 1))
            .next()
        {
            let copy_i = i - cycle_len;
            let rem_cycles = (1_000_000_000 - copy_i) % cycle_len;
            prev = cycle_n(&current, rem_cycles);
            break;
        }

        prev = current;
        history.push(prev.clone());
    }

    count(&prev)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Round,
    Cube,
}

fn cycle_n(grid: &Grid<Tile>, count: usize) -> Grid<Tile> {
    let mut current = grid.clone();
    for _ in 0..count {
        current = cycle(&current);
    }
    current
}

fn cycle(grid: &Grid<Tile>) -> Grid<Tile> {
    let grid = tilt(&grid, Direction::North);
    let grid = tilt(&grid, Direction::West);
    let grid = tilt(&grid, Direction::South);
    let grid = tilt(&grid, Direction::East);
    grid
}

fn tilt(grid: &Grid<Tile>, direction: Direction) -> Grid<Tile> {
    let dir = direction.opposite();

    let mut new_grid = grid.clone();
    new_grid.map.clear();

    let grid_bounds = BoundingBox2 {
        highest: Point2 {
            x: grid.width as isize,
            y: grid.height as isize,
        },
        lowest: Point2::zero(),
    };

    let points: Vec<Vec<Point2>> = match dir {
        Direction::North => (0..grid.width)
            .map(|x| {
                (0..grid.height)
                    .rev()
                    .map(move |y| Point2 {
                        x: x as isize,
                        y: y as isize,
                    })
                    .collect()
            })
            .collect(),
        Direction::East => (0..grid.height)
            .map(|y| {
                (0..grid.width)
                    .map(move |x| Point2 {
                        x: x as isize,
                        y: y as isize,
                    })
                    .collect()
            })
            .collect(),
        Direction::South => (0..grid.width)
            .map(|x| {
                (0..grid.height)
                    .map(move |y| Point2 {
                        x: x as isize,
                        y: y as isize,
                    })
                    .collect()
            })
            .collect(),
        Direction::West => (0..grid.height)
            .map(|y| {
                (0..grid.width)
                    .rev()
                    .map(move |x| Point2 {
                        x: x as isize,
                        y: y as isize,
                    })
                    .collect()
            })
            .collect(),
        _ => panic!("invalid direction"),
    };

    for line in points {
        let points = line
            .into_iter()
            .filter_map(|p| grid.map.get_key_value(&p))
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<_>>();
        let points = arrange_points(points, grid_bounds, dir);
        for p in points {
            new_grid.map.insert(p.0, p.1);
        }
    }

    new_grid
}

fn count(grid: &Grid<Tile>) -> usize {
    grid.map
        .iter()
        .filter_map(|(p, t)| (*t == Tile::Round).then_some(grid.height - (p.y as usize)))
        .sum()
}

fn point_on_edge(other: Point2, size: BoundingBox2, direction: Direction) -> Point2 {
    match direction {
        Direction::North => Point2 {
            x: other.x,
            y: size.highest.y - 1,
        },
        Direction::East => Point2 { x: 0, y: other.y },
        Direction::South => Point2 { x: other.x, y: 0 },
        Direction::West => Point2 {
            x: size.highest.x - 1,
            y: other.y,
        },
        _ => panic!("invalid direction"),
    }
}

fn arrange_points(
    points: Vec<(Point2, Tile)>,
    bounds: BoundingBox2,
    direction: Direction,
) -> Vec<(Point2, Tile)> {
    let Some((first_point, _)) = points.first() else {
        return Vec::new();
    };
    let mut fall_to = point_on_edge(*first_point, bounds, direction);
    let mut target = Vec::new();
    for (p, tile) in points {
        match tile {
            Tile::Round => {
                target.push((fall_to, tile));
                fall_to = fall_to + direction.offset();
            }
            Tile::Cube => {
                fall_to = p + direction.offset();
                target.push((p, tile));
            }
        }
    }
    target
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Round => 'O',
                Tile::Cube => '#',
            }
        )
    }
}

fn parse(input: &str) -> Grid<Tile> {
    Grid::from_vec(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => Some(Tile::Cube),
                        'O' => Some(Tile::Round),
                        _ => None,
                    })
                    .collect()
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/14.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(109466, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(94585, super::two(&input));
    }
}
