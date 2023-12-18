use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, HashMap, HashSet},
};

use itertools::Itertools;
use pathfinding::{directed::bfs::bfs_reach, matrix::Matrix, num_traits::Pow};

use crate::utils::grid::{Direction, Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let steps: Vec<Step> = input.lines().map(decode_step1).collect();
    let mut map = HashSet::new();

    let mut start = Point2::zero();
    map.insert(start);
    for s in steps {
        for _ in 0..s.meters {
            start = start + s.dir.offset();
            map.insert(start);
        }
    }

    let interior = interior_points(&map);

    interior.union(&map).count()
}

// #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
// struct X(isize);
// #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
// struct Y(isize);

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let steps: Vec<Step> = input.lines().map(decode_step1).collect();
    let mut lines: HashMap<isize, Vec<isize>> = HashMap::new();
    let mut points = Vec::new();

    eprintln!("parsing map ...");

    let mut point = Point2::zero();
    for s in steps {
        for _ in 0..s.meters {
            point = point + s.dir.offset();
            points.push(point);
            // normals.entry(point.y).or_default().push(point.x);
        }
    }

    let set = HashSet::from_iter(points.iter().copied());

    eprintln!("creating normals ...");

    let normals = normals(&points, &set);

    let print_normals = Grid {
        width: 0,
        height: 0,
        map: normals
            .iter()
            .map(|(k, v)| {
                (
                    *k,
                    match Direction::of(*v) {
                        Direction::North => '^',
                        Direction::East => '>',
                        Direction::South => 'V',
                        Direction::West => '<',
                        _ => panic!("invalid dir"),
                    },
                )
            })
            .collect(),
    };

    eprintln!("normals:\n{print_normals}");

    // for (_, v) in normals.iter_mut() {
    //     v.sort();
    // }

    let pts = points_on_edge(&set);
    dbg!(&pts);

    eprintln!("building interior path ...");

    let map_lining = interior_circumference(&set);

    // let mut lining_grid = Grid {
    //     height: 0,
    //     width: 0,
    //     map: map_lining.clone().into_iter().map(|k| (k, 'X')).collect(),
    // };
    // for p in &set {
    //     lining_grid.map.insert(*p, '#');
    // }
    // eprintln!("{lining_grid}");

    // dbg!(&map_lining);

    eprintln!("scanning ...");

    // Scan by line
    let mut total_len = dbg!(set.len());
    for y in *lines.keys().min().unwrap()..=*lines.keys().max().unwrap() {
        for (a, b) in lines
            .get(&y)
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(a, b)| (Point2 { y, x: *a }, Point2 { y, x: *b }))
        {
            let len = ((b - a).x - 1) as usize;
            if len > 0 {
                let in_map = map_lining.contains(&(a + Point2 { x: 1, y: 0 }));
                if in_map {
                    total_len += len;
                }
            }
        }
    }

    total_len
}

fn normals(points: &Vec<Point2>, set: &HashSet<Point2>) -> HashMap<Point2, Point2> {
    let point_along_east_edge = *set
        .iter()
        .max_set_by_key(|&k| k.x)
        .into_iter()
        .filter(|&p| {
            let east = *p + Direction::East.offset();
            let west = *p + Direction::West.offset();
            !set.contains(&east) && !set.contains(&west)
        })
        .next()
        .unwrap();

    let mut points = points.clone();
    let idx = points
        .iter()
        .position(|p| *p == point_along_east_edge)
        .unwrap();
    points.rotate_left(idx);

    let cur = points.iter().next().unwrap();
    let nxt = points.iter().skip(1).next().unwrap();
    let dir = Direction::of(*nxt - *cur);

    assert!(dir == Direction::South || dir == Direction::North);

    let rotate = if dir == Direction::South {
        // Inside is direction vector turned 270 degrees counter-clockwise (90 degrees clockwise)
        Matrix::from_vec(2, 2, vec![0, 1, -1, 0]).unwrap()
    } else {
        // Inside is direction vector turned 90 degrees counter-clockwise.
        Matrix::from_vec(2, 2, vec![0, -1, 1, 0]).unwrap()
    };

    let mut map = HashMap::new();
    // Going along the path in `dir`, West is inside, East is outside
    for (&a, &b) in points.iter().tuple_windows() {
        let dir = b - a;
        let normal = dir * &rotate;
        map.insert(a, normal);
    }

    map
}

fn interior_circumference(set: &HashSet<Point2>) -> HashSet<Point2> {
    let mut points = HashSet::new();

    let edge_points = points_on_edge(&set);
    for p in edge_points {
        points.extend(bfs_reach(p, |&p| {
            Direction::cardinals()
                .map(|d| p + d.offset())
                .into_iter()
                .filter(|p| {
                    if set.contains(p) {
                        false
                    } else {
                        let is_next_to_path = Direction::cardinals()
                            .map(|d| *p + d.offset())
                            .iter()
                            .any(|p| set.contains(p));

                        is_next_to_path
                    }
                })
        }));
    }

    points

    // let bottom_most = bottom_most_inner_point(set);

    // bfs_reach(bottom_most, |&p| {
    //     Direction::cardinals()
    //         .map(|d| p + d.offset())
    //         .into_iter()
    //         .filter(|p| {
    //             if set.contains(p) {
    //                 false
    //             } else {
    //                 let is_next_to_path = Direction::cardinals()
    //                     .map(|d| *p + d.offset())
    //                     .iter()
    //                     .any(|p| set.contains(p));

    //                 is_next_to_path
    //             }
    //         })
    // })
    // .collect()
}

fn interior_points(set: &HashSet<Point2>) -> HashSet<Point2> {
    let bottom_most = bottom_most_inner_point(set);

    bfs_reach(bottom_most, |&p| {
        Direction::cardinals()
            .map(|d| p + d.offset())
            .into_iter()
            .filter(|p| !set.contains(p))
    })
    .collect()
}

fn points_on_edge(set: &HashSet<Point2>) -> [Point2; 4] {
    let edges = [
        (Direction::South, set.iter().min_set_by_key(|&p| p.y)),
        (Direction::West, set.iter().max_set_by_key(|&p| p.x)),
        (Direction::North, set.iter().max_set_by_key(|&p| p.y)),
        (Direction::East, set.iter().min_set_by_key(|&p| p.x)),
    ];
    let points = edges.map(|(d, e)| {
        e.iter()
            .filter_map(|&p| {
                let nxt = *p + d.offset();
                if set.contains(&nxt) {
                    None
                } else {
                    Some(nxt)
                }
            })
            .next()
            .unwrap()
    });
    points
}

fn bottom_most_inner_point(set: &HashSet<Point2>) -> Point2 {
    let bottom_most = set.iter().max_set_by_key(|&&k| k.y);
    bottom_most
        .into_iter()
        .sorted_by_key(|p| Reverse(p.x))
        .filter_map(|&p| {
            let north_of = p + Direction::North.offset();
            if set.contains(&north_of) {
                None
            } else {
                Some(north_of)
            }
        })
        .next()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Step {
    dir: Direction,
    meters: u32,
}

fn decode_step1(s: &str) -> Step {
    let mut parts = s.trim().split_whitespace();
    let dir = parts
        .next()
        .map(|c| match c {
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            "U" => Direction::North,
            _ => panic!("unknown direction"),
        })
        .unwrap();
    let meters = parts.next().map(|n| n.parse().unwrap()).unwrap();
    Step { dir, meters }
}

fn decode_step2(s: &str) -> Step {
    let hex = s
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .strip_prefix("(#")
        .unwrap()
        .strip_suffix(')')
        .unwrap();
    let dir = match hex.chars().last().unwrap() {
        '0' => Direction::East,
        '1' => Direction::South,
        '2' => Direction::West,
        '3' => Direction::North,
        _ => panic!("unknown direction"),
    };
    let meters = u32::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();
    Step { dir, meters }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/18.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(50746, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(0, super::two(&input));
    }
}
