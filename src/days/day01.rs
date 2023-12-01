#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let nums: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    input
        .lines()
        .map(|ll| {
            let mut l = ll;
            let mut v = Vec::new();
            loop {
                if l.is_empty() {
                    break;
                }
                if let Some(n) = next_num(l) {
                    v.push(n);
                }
                l = &l[1..];
            }
            v
        })
        .map(|v| v.first().unwrap() * 10 + v.last().unwrap())
        .sum()
}

fn next_num(input: &str) -> Option<u32> {
    if let Some(n @ Some(_)) = input.chars().next().map(|c| c.to_digit(10)) {
        n
    } else if input.starts_with("one") {
        Some(1)
    } else if input.starts_with("two") {
        Some(2)
    } else if input.starts_with("three") {
        Some(3)
    } else if input.starts_with("four") {
        Some(4)
    } else if input.starts_with("five") {
        Some(5)
    } else if input.starts_with("six") {
        Some(6)
    } else if input.starts_with("seven") {
        Some(7)
    } else if input.starts_with("eight") {
        Some(8)
    } else if input.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/01.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(55607, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(55291, super::two(&input));
    }
}
