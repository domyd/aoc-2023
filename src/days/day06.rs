use std::str::FromStr;

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> u64 {
    let races = input.parse::<Event>().unwrap().0;
    races
        .into_iter()
        .map(|r| {
            (1..r.time)
                .map(|t| r.simulate(t))
                .filter(|t| *t > r.record_dist)
                .count() as u64
        })
        .product()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u64 {
    let races = input.replace(' ', "").parse::<Event>().unwrap().0;
    let race = races.first().unwrap();
    (1..race.time)
        .map(|t| race.simulate(t))
        .filter(|t| *t > race.record_dist)
        .count() as u64
}

#[derive(Clone, Debug)]
struct Event(Vec<Race>);

#[derive(Clone, Copy, Debug)]
struct Race {
    time: u64,
    record_dist: u64,
}

impl Race {
    pub fn simulate(&self, time: u64) -> u64 {
        if time == 0 || time >= self.time {
            0
        } else {
            let remaining = self.time - time;
            let speed = time;
            remaining * speed
        }
    }
}

impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn nums(s: &str) -> Vec<u64> {
            s.split_once(':')
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        }

        let (times, distances) = s.split_once('\n').unwrap();
        let (times, distances) = (nums(times), nums(distances));

        Ok(Event(
            times
                .into_iter()
                .zip_eq(distances)
                .map(|(time, record_dist)| Race { time, record_dist })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/06.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(505494, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(23632299, super::two(&input));
    }
}
