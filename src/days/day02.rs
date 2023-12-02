use std::str::FromStr;

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .filter(|g| {
            g.hands
                .iter()
                .all(|h| h.red <= 12 && h.green <= 13 && h.blue <= 14)
        })
        .map(|g| g.id)
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .map(|g| {
            let min_bag: Hand = g
                .hands
                .iter()
                .fold(Hand::new(), |acc, x| acc.component_max(x));
            min_bag.red * min_bag.green * min_bag.blue
        })
        .sum()
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, hands) = s.split_once(':').unwrap();
        let id = game.strip_prefix("Game ").unwrap().parse().unwrap();
        let hands = hands.split(';').map(|s| s.parse().unwrap()).collect();
        Ok(Game { id, hands })
    }
}

#[derive(Clone, Copy, Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn component_max(&self, other: &Hand) -> Self {
        Hand {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for part in s.split(',') {
            let (num, col) = part.trim().split_once(' ').unwrap();
            let num = num.parse::<u32>().unwrap();

            match col {
                "red" => {
                    red = num;
                }
                "green" => {
                    green = num;
                }
                "blue" => {
                    blue = num;
                }
                _ => unreachable!(),
            }
        }

        Ok(Hand { red, green, blue })
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/02.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(2416, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(63307, super::two(&input));
    }
}
