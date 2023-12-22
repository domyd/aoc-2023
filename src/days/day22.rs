use std::collections::HashSet;

use itertools::Itertools;

use self::tetris::{Brick, Tetris};

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let bricks = initial_bricks(input);

    let mut counter = 0;
    for i in 0..bricks.len() {
        let mut test = bricks.clone();
        test.remove(i);
        let fallen = Tetris::new(&test).into_iter().collect::<Vec<_>>();
        if fallen == test {
            counter += 1;
        }
    }

    counter
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let bricks = initial_bricks(input);

    let mut counter = 0;
    for i in 0..bricks.len() {
        let mut test = bricks.clone();
        test.remove(i);

        let fallen: HashSet<Brick> = Tetris::new(&test).into_iter().collect();
        let test = HashSet::<_>::from_iter(test.iter().copied());
        let fallen = test.difference(&fallen).count();

        counter += fallen;
    }

    counter
}

fn initial_bricks(input: &str) -> Vec<Brick> {
    let mut bricks = parse(input);
    bricks.sort_by_key(|b| b.bbox.lowest.z);
    Tetris::new(&bricks).into_iter().collect()
}

fn parse(input: &str) -> Vec<Brick> {
    fn parse_point(input: &str) -> crate::utils::grid::Point3 {
        let (x, y, z) = input
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        crate::utils::grid::Point3 { x, y, z }
    }

    input
        .lines()
        .enumerate()
        .map(|(id, l)| {
            let (lowest, highest) = l
                .split_once('~')
                .map(|(a, b)| (parse_point(a), parse_point(b)))
                .unwrap();
            tetris::Brick {
                id,
                bbox: crate::utils::grid::BoundingBox3 { lowest, highest },
            }
        })
        .collect()
}

mod tetris {
    use crate::utils::grid::{BoundingBox2, BoundingBox3, Point2, Point3};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Brick {
        pub id: usize,
        pub bbox: BoundingBox3,
    }

    pub struct Tetris<'a> {
        bricks: &'a [Brick],
    }

    impl<'a> Tetris<'a> {
        pub fn new(bricks: &'a [Brick]) -> Self {
            Tetris { bricks }
        }
    }

    pub struct TetrisIter<'a> {
        tetris: Tetris<'a>,
        floor: [usize; 10 * 10],
        i: usize,
    }

    impl<'a> IntoIterator for Tetris<'a> {
        type Item = Brick;
        type IntoIter = TetrisIter<'a>;

        fn into_iter(self) -> Self::IntoIter {
            TetrisIter {
                tetris: self,
                floor: [0; 10 * 10],
                i: 0,
            }
        }
    }

    impl<'a> Iterator for TetrisIter<'a> {
        type Item = Brick;

        fn next(&mut self) -> Option<Self::Item> {
            let brick = self.tetris.bricks.get(self.i)?;
            self.i += 1;

            let mut volume = brick.bbox;

            loop {
                let bottoms = bbox_floor(&volume)
                    .into_iter()
                    .map(|b| {
                        (
                            (b.y * 10 + b.x) as usize,
                            (volume.lowest.z as usize, volume.highest.z as usize),
                        )
                    })
                    .collect::<Vec<_>>();

                if bottoms.iter().any(|(i, (l, _))| self.floor[*i] + 1 == *l) {
                    // stop the brick from falling and update floors
                    for (i, (_, h)) in bottoms.iter() {
                        self.floor[*i] = *h;
                    }
                    return Some(Brick {
                        bbox: volume,
                        ..*brick
                    });
                }

                volume = volume.translate(&Point3 { x: 0, y: 0, z: -1 });
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.len(), Some(self.len()))
        }
    }

    impl<'a> ExactSizeIterator for TetrisIter<'a> {
        fn len(&self) -> usize {
            self.tetris.bricks.len() - self.i
        }
    }

    fn bbox_floor(brick: &BoundingBox3) -> BoundingBox2 {
        BoundingBox2 {
            lowest: Point2 {
                x: brick.lowest.x,
                y: brick.lowest.y,
            },
            highest: Point2 {
                x: brick.highest.x,
                y: brick.highest.y,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/22.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(443, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(69915, super::two(&input));
    }
}
