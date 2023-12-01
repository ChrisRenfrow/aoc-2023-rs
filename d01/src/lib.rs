/// Finds the first digit and the last digit on each line to form a two-digit number and sums them all together
pub fn d01p1(input: String) -> u32 {
    input.lines().fold(0, |acc, line| {
        let first = line.chars().find_map(|s| s.to_digit(10));
        let last = line.chars().rev().find_map(|s| s.to_digit(10));
        // We're guaranteed a result for first and last so it's safe to unwrap
        acc + first.unwrap() * 10 + last.unwrap()
    })
}

pub fn d01p2(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
    const P2_INPUT: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    const P1_ANSWER: u32 = 142;
    const P2_ANSWER: u32 = 281;

    #[test]
    fn part_one() {
        assert!(d01p1(P1_INPUT.to_string()) == P1_ANSWER)
    }

    #[test]
    fn part_two() {
        assert!(d01p2(P2_INPUT.to_string()) == P2_ANSWER)
    }
}
