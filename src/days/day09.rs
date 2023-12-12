use std::str::FromStr;

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse::<History>().unwrap())
        .map(|h| h.predict_next())
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut history = l.parse::<History>().unwrap();
            history.0.reverse();
            history
        })
        .map(|h| h.predict_next())
        .sum()
}

#[derive(Debug, Clone)]
struct History(Vec<i64>);

impl History {
    fn derivations(&self) -> Vec<Vec<i64>> {
        let mut dxs = Vec::new();
        dxs.push(self.0.clone());
        loop {
            let derived = dxs
                .last()
                .unwrap()
                .iter()
                .zip(dxs.last().unwrap().iter().skip(1))
                .map(|(l, r)| *r - *l)
                .collect_vec();

            if derived.iter().all(|x| *x == 0) {
                dxs.push(derived);
                break;
            }

            dxs.push(derived);
        }

        dxs
    }

    pub fn predict_next(&self) -> i64 {
        let mut dxs = self.derivations();
        dxs.reverse();
        dxs.iter().skip(1).fold(0, |acc, x| x.last().unwrap() + acc)
    }
}

impl FromStr for History {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(History(
            s.split_whitespace().map(|s| s.parse().unwrap()).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/09.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(1853145119, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(923, super::two(&input));
    }
}
