use std::{
    collections::{HashMap, VecDeque},
    ops::{Neg, Range},
    str::FromStr,
};

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let (workflows, parts) = parse(input);
    let mut results = Vec::new();

    for part in parts {
        let mut queue = VecDeque::new();
        queue.push_back(workflows.get("in").unwrap());

        while let Some(wf) = queue.pop_front() {
            match wf.inspect(part) {
                Then::Workflow(wf) => queue.push_back(workflows.get(&wf).unwrap()),
                Then::Accept => results.push(part),
                Then::Reject => {}
            }
        }
    }

    results
        .iter()
        .map(|part| (part.x + part.m + part.a + part.s) as usize)
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let (workflows, _) = parse(input);
    let mut finished = Vec::new();

    eval_all_parts(
        &workflows,
        workflows.get("in").unwrap(),
        PartsRange::new(),
        &mut finished,
    );

    finished.iter().map(|state| state.len()).sum()
}

fn eval_all_parts(
    workflows: &HashMap<String, Workflow>,
    workflow: &Workflow,
    range: PartsRange,
    finished: &mut Vec<PartsRange>,
) {
    let mut range = range.clone();

    for check in &workflow.0 {
        let (clamped, next) = if let Some(cond) = check.condition {
            let (var, cond) = match cond {
                If::Lt(var, n) => (var, HalfRange::LessThan(n)),
                If::Gt(var, n) => (var, HalfRange::GreaterThan(n)),
            };
            (range.clamp(var, cond), range.clamp(var, cond.neg()))
        } else {
            (range.clone(), range.clone())
        };

        match &check.execute {
            Then::Workflow(wf) => eval_all_parts(
                workflows,
                workflows.get(wf).unwrap(),
                clamped.clone(),
                finished,
            ),
            Then::Accept => finished.push(clamped),
            Then::Reject => {}
        };

        range = next;
    }
}

#[derive(Debug, Clone)]
struct PartsRange {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl PartsRange {
    pub fn new() -> PartsRange {
        let range = 1u32..4001u32;
        PartsRange {
            x: range.clone(),
            m: range.clone(),
            a: range.clone(),
            s: range,
        }
    }

    pub fn clamp(&self, var: Var, half_range: HalfRange) -> PartsRange {
        let clamp_range = |range: &Range<u32>| match half_range {
            HalfRange::LessThan(n) => range.start..(n.min(range.end)),
            HalfRange::GreaterThan(n) => ((n + 1).max(range.start))..range.end,
            HalfRange::LessThanOrEqual(n) => range.start..((n + 1).min(range.end)),
            HalfRange::GreaterThanOrEqual(n) => (n.max(range.start))..range.end,
        };

        let mut new = self.clone();

        match var {
            Var::X => new.x = clamp_range(&self.x),
            Var::M => new.m = clamp_range(&self.m),
            Var::A => new.a = clamp_range(&self.a),
            Var::S => new.s = clamp_range(&self.s),
        }

        new
    }

    pub fn len(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug, Clone, Copy)]
enum HalfRange {
    LessThan(u32),
    GreaterThan(u32),
    LessThanOrEqual(u32),
    GreaterThanOrEqual(u32),
}

impl Neg for HalfRange {
    type Output = HalfRange;

    fn neg(self) -> Self::Output {
        match self {
            HalfRange::LessThan(n) => HalfRange::GreaterThanOrEqual(n),
            HalfRange::GreaterThan(n) => HalfRange::LessThanOrEqual(n),
            HalfRange::LessThanOrEqual(n) => HalfRange::GreaterThan(n),
            HalfRange::GreaterThanOrEqual(n) => HalfRange::LessThan(n),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    pub fn var(&self, var: Var) -> u32 {
        match var {
            Var::X => self.x,
            Var::M => self.m,
            Var::A => self.a,
            Var::S => self.s,
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow(Vec<Step>);

impl Workflow {
    pub fn inspect(&self, part: Part) -> Then {
        for check in &self.0 {
            if let Some(cond) = check.condition {
                if match cond {
                    If::Lt(var, n) => part.var(var) < n,
                    If::Gt(var, n) => part.var(var) > n,
                } {
                    return check.execute.clone();
                }
            } else {
                return check.execute.clone();
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Clone)]
struct Step {
    condition: Option<If>,
    execute: Then,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Var {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
enum If {
    Lt(Var, u32),
    Gt(Var, u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Then {
    Workflow(String),
    Accept,
    Reject,
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(',')
            .map(|part| part.parse())
            .collect::<Result<Vec<Step>, _>>()?;
        Ok(Workflow(parts))
    }
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (condition, execute) = s
            .split_once(':')
            .map_or_else(|| (None, s), |(a, b)| (Some(a), b));

        let execute = match execute {
            "A" => Then::Accept,
            "R" => Then::Reject,
            s => Then::Workflow(s.to_string()),
        };

        let condition = condition
            .map(|c| {
                if let Some((a, b)) = c.split_once('<') {
                    let var = a.parse::<Var>().unwrap();
                    Some(If::Lt(var, b.parse().unwrap()))
                } else if let Some((a, b)) = c.split_once('>') {
                    let var = a.parse::<Var>().unwrap();
                    Some(If::Gt(var, b.parse().unwrap()))
                } else {
                    None
                }
            })
            .flatten();

        Ok(Step { condition, execute })
    }
}

impl FromStr for Var {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "X" | "x" => Ok(Var::X),
            "M" | "m" => Ok(Var::M),
            "A" | "a" => Ok(Var::A),
            "S" | "s" => Ok(Var::S),
            _ => Err("invalid variable".to_string()),
        }
    }
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let input = input.replace('\r', "");
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|l| {
            l.split_once('{')
                .map(|(a, b)| {
                    (
                        a.to_string(),
                        b.strip_suffix('}').unwrap().parse::<Workflow>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();

    let parts = parts
        .lines()
        .map(|l| {
            let (x, m, a, s) = l
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|part| part.split_once('=').unwrap().1.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Part { x, m, a, s }
        })
        .collect();

    (workflows, parts)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/19.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(406849, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(138625360533574, super::two(&input));
    }
}
