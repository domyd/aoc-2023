use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    hash::Hash,
    iter,
    ops::Rem,
};

use indexmap::IndexMap;
use itertools::Itertools;
use pathfinding::directed::bfs::bfs_reach;

use crate::utils::grid::{BoundingBox2, Direction, Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let grid = parse(input);

    let start = Node(0, grid.find(|t| *t == Tile::Start).map(|(p, _)| p).unwrap());
    let blocked = grid
        .map
        .iter()
        .filter_map(|(p, t)| (*t == Tile::Rock).then_some(p))
        .collect::<HashSet<_>>();

    let count = 64;

    let bbox = BoundingBox2 {
        lowest: Point2::zero(),
        highest: Point2 {
            x: (grid.width as isize) - 1,
            y: (grid.height as isize - 1),
        },
    };

    let reached = bfs_reach(start, |n| {
        if n.0 > count {
            vec![]
        } else {
            Direction::cardinals()
                .map(|d| n.1 + d.offset())
                .into_iter()
                .filter_map(|p| {
                    if bbox.contains(&p) && !blocked.contains(&p) {
                        Some(Node(n.0 + 1, p))
                    } else {
                        None
                    }
                })
                .collect()
        }
    });

    reached.filter(|n| n.0 == count).count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TiledPoint2 {
    maps_to: Point2,
    quadrant: Option<Direction>,
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let grid = parse(input);
    let start = grid.find(|t| *t == Tile::Start).map(|(p, _)| p).unwrap();
    let blocked = grid
        .map
        .iter()
        .filter_map(|(p, t)| (*t == Tile::Rock).then_some(p))
        .collect::<HashSet<_>>();

    let map_point = |p: Point2| -> TiledPoint2 {
        let maps_to = Point2 {
            x: p.x.rem_euclid(grid.width as isize),
            y: p.y.rem_euclid(grid.height as isize),
        };
        let quadrant = if maps_to == p {
            None
        } else {
            Some(if p.x < 0 {
                if p.y < 0 {
                    Direction::NorthWest
                } else if p.y < grid.height as isize {
                    Direction::West
                } else {
                    Direction::SouthWest
                }
            } else if p.x < grid.width as isize {
                if p.y < 0 {
                    Direction::North
                } else {
                    Direction::South
                }
            } else {
                if p.y < 0 {
                    Direction::NorthEast
                } else if p.y < grid.height as isize {
                    Direction::East
                } else {
                    Direction::SouthEast
                }
            })
        };
        TiledPoint2 { maps_to, quadrant }
    };

    let mut dist: HashMap<Point2, HashMap<Direction, Vec<usize>>> = HashMap::new();

    let threebythree = BoundingBox2 {
        lowest: Point2 {
            x: -(grid.width as isize),
            y: -(grid.width as isize),
        },
        highest: Point2 {
            x: ((grid.width as isize) * 2) - 1,
            y: ((grid.width as isize) * 2) - 1,
        },
    };

    let distances = distances(start, |p| {
        Direction::cardinals()
            .map(|d| *p + d.offset())
            .iter()
            .filter_map(|p| {
                let mp = map_point(*p);
                if !threebythree.contains(p) || blocked.contains(&mp.maps_to) {
                    None
                } else {
                    Some(*p)
                }
            })
            .collect_vec()
    });

    for (p, d) in distances.iter() {
        let tiled = map_point(*p);
        let v = dist.entry(tiled.maps_to).or_default();
        // let v = v.entry(tiled.quadrant).or_default();
    }

    dbg!(&dist.get(&Point2 { x: 9, y: 4 }));

    // eprintln!("{:?}", &res);

    todo!()

    // let blocked = grid
    //     .map
    //     .iter()
    //     .filter_map(|(p, t)| (*t == Tile::Rock).then_some(p))
    //     .collect::<HashSet<_>>();

    // let map_point = |p: Point2| -> Point2 {
    //     Point2 {
    //         x: p.x.rem_euclid(grid.width as isize),
    //         y: p.y.rem_euclid(grid.height as isize),
    //     }
    // };

    // let bigbbox = BoundingBox2 {
    //     lowest: Point2 {
    //         x: -5_000,
    //         y: -5_000,
    //     },
    //     highest: Point2 { x: 5_000, y: 5_000 },
    // };

    // let reachable = bfs_reach(start.1, |p| {
    //     Direction::cardinals()
    //         .map(|d| *p + d.offset())
    //         .into_iter()
    //         .filter(|p| !blocked.contains(p) && bigbbox.contains(p))
    // });

    // dbg!(reachable.count());

    // let reached = bfs_reach(start, |n| {
    //     if n.0 > count {
    //         vec![]
    //     } else {
    //         Direction::cardinals()
    //             .map(|d| n.1 + d.offset())
    //             .into_iter()
    //             .filter_map(|p| {
    //                 if bbox.contains(&p) && !blocked.contains(&p) {
    //                     Some(Node(n.0 + 1, p))
    //                 } else {
    //                     None
    //                 }
    //             })
    //             .collect()
    //     }
    // });

    // reached.filter(|n| n.0 == count).count()
}

fn distances<N, FN, IN>(start: N, mut successors: FN) -> HashMap<N, usize>
where
    N: Eq + Hash + Clone + std::fmt::Debug,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut results: HashMap<N, usize> = HashMap::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0usize, start.clone()));

    while let Some((d, node)) = queue.pop_front() {
        if seen.contains(&node) {
            continue;
        } else {
            seen.insert(node.clone());
            results.insert(node.clone(), d);
        }

        for successor in successors(&node) {
            // if seen.contains(&successor) {
            //     continue;
            // }

            queue.push_back((d + 1, successor));
        }
    }

    results
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node(u32, Point2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Plot,
    Rock,
}

fn parse(input: &str) -> Grid<Tile> {
    Grid::from_vec(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Some(Tile::Plot),
                        '#' => Some(Tile::Rock),
                        'S' => Some(Tile::Start),
                        _ => panic!("unknown tile"),
                    })
                    .collect()
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/21.txt";

    #[test]
    #[ignore]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(3847, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(0, super::two(&input));
    }
}
