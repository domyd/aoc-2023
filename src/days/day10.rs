#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    unimplemented!()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/10.txt";

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
