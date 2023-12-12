use std::{collections::HashMap, str::FromStr};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let network: Network = input.parse().unwrap();
    let mut node = String::from("AAA");
    let mut count = 0u32;
    for instr in network.instr.iter().cycle() {
        if node == "ZZZ" {
            break;
        }

        node = network
            .map
            .get(&node)
            .map(|(l, r)| match instr {
                Instr::Left => l,
                Instr::Right => r,
            })
            .unwrap()
            .to_string();
        count += 1;
    }
    count
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let network: FastNetwork = input.parse::<Network>().unwrap().into();
    let mut nodes: Vec<isize> = network.start_idxs.clone();
    let mut count = 0u32;
    for instr in network.instr.iter().cycle() {
        if count % 1_000_000 == 0 {
            eprintln!("{count}");
        }

        if nodes.iter().all(|n| network.end_idxs.contains(n)) {
            break;
        }

        nodes = nodes
            .into_iter()
            .map(|n| {
                let (offset_l, offset_r) = network.offsets.get(&n).unwrap();
                n + match instr {
                    Instr::Left => offset_l,
                    Instr::Right => offset_r,
                }
            })
            .collect();

        // let new_nodes = nodes
        //     .drain(..)
        //     .map(|n| {
        //         network
        //             .map
        //             .get(&n)
        //             .map(|(l, r)| match instr {
        //                 Instr::Left => l,
        //                 Instr::Right => r,
        //             })
        //             .unwrap()
        //             .to_string()
        //     })
        //     .collect::<Vec<_>>();
        // nodes.extend(new_nodes);

        count += 1;
    }
    count
}

#[derive(Clone, Copy, Debug)]
enum Instr {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Network {
    instr: Vec<Instr>,
    map: HashMap<String, (String, String)>,
}

struct FastNetwork {
    instr: Vec<Instr>,
    start_idxs: Vec<isize>,
    end_idxs: Vec<isize>,
    offsets: HashMap<isize, (isize, isize)>,
}

impl From<Network> for FastNetwork {
    fn from(value: Network) -> Self {
        todo!()
    }
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("\r\n", "\n");
        let (instr, map) = s.split_once("\n\n").unwrap();
        let instr = instr
            .trim()
            .chars()
            .map(|c| match c {
                'L' => Instr::Left,
                'R' => Instr::Right,
                _ => panic!("wrong instr char"),
            })
            .collect();

        let map = map
            .trim()
            .lines()
            .map(|l| {
                let l = l.replace('(', "").replace(')', "").replace(' ', "");
                let (key, (from, to)) = l
                    .split_once('=')
                    .map(|(k, rest)| (k, rest.split_once(',').unwrap()))
                    .unwrap();
                (key.to_string(), (from.to_string(), to.to_string()))
            })
            .collect();

        Ok(Network { instr, map })
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/08.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(15517, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(0, super::two(&input));
    }
}
