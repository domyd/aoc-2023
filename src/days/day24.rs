use itertools::Itertools;

use crate::utils::grid::{Point3, Point3f};

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    parse(input)
        .into_iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            intersect(*a, *b).map_or(false, |i| {
                i.x >= 200_000_000_000_000f64
                    && i.x <= 400_000_000_000_000f64
                    && i.y >= 200_000_000_000_000f64
                    && i.y <= 400_000_000_000_000f64
            })
        })
        .count()
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    unimplemented!()
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    origin: Point3,
    direction: Point3,
}

fn intersect(a: Ray, b: Ray) -> Option<Point3f> {
    todo!()
}

fn parse(input: &str) -> Vec<Ray> {
    fn parse_point(input: &str) -> Point3 {
        let (x, y, z) = input
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Point3 { x, y, z }
    }

    input
        .lines()
        .map(|l| {
            let (origin, direction) = l
                .split_once(" @ ")
                .map(|(a, b)| (parse_point(a), parse_point(b)))
                .unwrap();
            Ray { origin, direction }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/24.txt";

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
