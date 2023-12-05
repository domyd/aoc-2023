use std::{collections::BTreeMap, str::FromStr};

#[allow(dead_code)]
pub fn one(input: &str) -> u64 {
    let almanac: Almanac = input.parse().unwrap();
    let seeds_for_locations = &almanac
        .seeds
        .iter()
        .map(|s| (almanac.translate(*s), s))
        .collect::<BTreeMap<_, _>>();

    *seeds_for_locations.first_key_value().unwrap().0
}

#[allow(dead_code)]
pub fn two(input: &str) -> u64 {
    let almanac = {
        let mut almanac: Almanac = input.parse().unwrap();
        almanac.reverse();
        almanac
    };
    let seeds = almanac.seeds_as_ranges();

    let mut loc = 0u64;
    let loc = loop {
        let seed = almanac.translate(loc);
        if seeds.iter().any(|r| r.contains(&seed)) {
            break loc;
        }

        loc += 1;
    };

    loc
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn translate(&self, src: u64) -> u64 {
        self.maps.iter().fold(src, |acc, map| map.convert(acc))
    }

    pub fn reverse(&mut self) {
        self.maps.reverse();
        for map in &mut self.maps {
            for range in &mut map.ranges {
                std::mem::swap(&mut range.dst_start, &mut range.src_start);
            }
        }
    }

    pub fn seeds_as_ranges(&self) -> Vec<std::ops::Range<u64>> {
        self.seeds
            .chunks_exact(2)
            .map(|c| {
                let start = *c.get(0).unwrap();
                let len = *c.get(1).unwrap();
                start..start + len
            })
            .collect()
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds: Vec<u64> = s
            .lines()
            .next()
            .unwrap()
            .strip_prefix("seeds:")
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let maps: Vec<Map> = s
            .split("\n\n")
            .skip(1)
            .map(|v| v.parse().unwrap())
            .collect();

        Ok(Almanac { seeds, maps })
    }
}

#[derive(Clone, Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn convert(&self, src: u64) -> u64 {
        for r in &self.ranges {
            let src_range = r.src_start..(r.src_start + r.len);
            if src_range.contains(&src) {
                let offset = src - src_range.start;
                return r.dst_start + offset;
            }
        }

        return src;
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            ranges: s.lines().filter_map(|l| l.parse().ok()).collect(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u64> = s
            .trim()
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| "couldn't parse string".to_string()))
            .collect::<Result<Vec<u64>, String>>()?;
        match nums[..] {
            [dst_start, src_start, len] => Ok(Range {
                dst_start,
                src_start,
                len,
            }),
            _ => Err("not a range".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/05.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(579439039, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(7873084, super::two(&input));
    }
}
