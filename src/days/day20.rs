use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let mut succ = HashMap::new();
    let mut modules = HashMap::new();
    let mut init = Vec::new();

    for line in input.lines() {
        let (src, dst) = line.split_once(" -> ").unwrap();

        if src == "broadcaster" {
            init.extend(dst.split(',').map(|s| s.trim().to_string()));
            continue;
        }

        succ.insert(src.to_string(), dst.to_string());
        let m = if src.starts_with('%') {
            Mod::FlipFlop(FlipFlop { state: false })
        } else {
            Mod::Conjunction(Conjunction {
                inputs: HashMap::new(),
            })
        };
        modules.insert(src.to_string(), m);
    }

    // let state: HashMap<String, bool> = HashMap::new();
    // let conj: HashMap<String, Vec<bool>> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.extend(init.iter().map(|target| Pulse {
        signal: false,
        source: "broadcaster".to_string(),
        target: target.clone(),
    }));

    let mut counter = 0;

    while let Some(pulse) = queue.pop_front() {
        dbg!(&pulse);

        counter += 1;

        let target = modules.get_mut(&pulse.target).unwrap();
        if let Some(signal) = target.pulse(&pulse) {
            queue.push_back(Pulse {
                signal,
                source: pulse.target.clone(),
                target: succ.get(&pulse.target).unwrap().clone(),
            })
        }
    }

    counter
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    unimplemented!()
}

#[derive(Debug, Clone)]
struct Pulse {
    signal: bool,
    source: String,
    target: String,
}

#[derive(Debug, Clone)]
struct FlipFlop {
    state: bool,
}

impl FlipFlop {
    fn pulse(&mut self, pulse: &Pulse) -> Option<bool> {
        match pulse.signal {
            true => None,
            false => {
                self.state = !self.state;
                Some(self.state)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    inputs: HashMap<String, bool>,
}

impl Conjunction {
    fn pulse(&mut self, pulse: &Pulse) -> Option<bool> {
        self.inputs
            .entry(pulse.source.clone())
            .and_modify(|s| *s = pulse.signal);

        if self.inputs.values().all(|s| *s) {
            Some(false)
        } else {
            Some(true)
        }
    }
}

#[derive(Debug, Clone)]
enum Mod {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl Mod {
    fn pulse(&mut self, pulse: &Pulse) -> Option<bool> {
        match self {
            Mod::FlipFlop(f) => f.pulse(pulse),
            Mod::Conjunction(c) => c.pulse(pulse),
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/20.txt";

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
