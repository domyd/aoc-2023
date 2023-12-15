use std::{collections::HashMap, str::FromStr};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    input
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .map(|s| hash(&s))
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let seq = input.parse::<InitSeq>().unwrap();
    let map = seq.run();
    map.iter()
        .map(|(&k, v)| {
            let b = (k as u32) + 1;
            b * v
                .iter()
                .enumerate()
                .map(|(slot, lens)| (slot as u32 + 1) * lens.focal_length)
                .sum::<u32>()
        })
        .sum()
}

fn hash(t: &str) -> u32 {
    t.chars().fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug, Clone)]
enum Instruction {
    Remove(String),
    Install(Lens),
}

#[derive(Debug, Clone)]
struct InitSeq(Vec<Instruction>);

impl InitSeq {
    pub fn run(&self) -> HashMap<u8, Vec<Lens>> {
        let mut map: HashMap<u8, Vec<Lens>> = HashMap::new();

        for instr in self.0.iter() {
            match instr {
                Instruction::Remove(label) => {
                    let bx = hash(&label).try_into().unwrap();
                    map.entry(bx).and_modify(|v| {
                        if let Some(idx) = v.iter().position(|l| l.label == *label) {
                            v.remove(idx);
                        }
                    });
                }
                Instruction::Install(lens) => {
                    let bx = hash(&lens.label).try_into().unwrap();
                    let contents = map.entry(bx).or_default();
                    let exists_at = contents.iter().position(|l| l.label == *lens.label);
                    contents.push(lens.clone());
                    if let Some(idx) = exists_at {
                        contents.swap_remove(idx);
                    }
                }
            }
        }

        map
    }
}

impl FromStr for InitSeq {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InitSeq(
            s.trim()
                .split(',')
                .map(|s| {
                    let (label, num) = s.split_once(&['-', '=']).unwrap();
                    if s.contains('-') {
                        Instruction::Remove(label.to_string())
                    } else {
                        Instruction::Install(Lens {
                            label: label.to_string(),
                            focal_length: num.parse().unwrap(),
                        })
                    }
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/15.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(521434, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(248279, super::two(&input));
    }
}
