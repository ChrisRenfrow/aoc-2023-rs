use std::collections::HashMap;

use regex::Regex;

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
    let num_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let num_re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();

    input.lines().fold(0, |acc, line| {
        let mut match_iter = num_re.find_iter(line);
        let first_match = match_iter.next();
        // Consume the iterator and return the last match
        let last_match = num_re.find_iter(line).last();
        let first_digit = line
            .chars()
            .enumerate()
            .find_map(|(i, c)| match c.to_digit(10) {
                Some(d) => Some((i, d)),
                None => None,
            });
        let last_digit = line.chars().rev().enumerate().find_map(|(i, c)| {
            match c.to_digit(10) {
                // We have to "fix" the index here since enumerate comes after reversing
                Some(d) => Some((line.len() - i, d)),
                None => None,
            }
        });
        // Here we figure out which kind of number should be used as first and last based on their order and existence
        let (first, last) = (
            match (first_digit, first_match) {
                // If the first char digit is present and occurs before string digit, use char digit
                (Some((i, d)), Some(f_mat)) if i < f_mat.start() => d,
                // If string digit exists use that
                (_, Some(f_mat)) => *num_map.get(f_mat.as_str()).unwrap(),
                // Default to char digit
                (Some((_, d)), None) => d,
                _ => unreachable!(),
            },
            match (last_digit, last_match) {
                // If the last char digit is present and occurs after string digit, use char digit
                (Some((i, d)), Some(l_mat)) if i > l_mat.start() => d,
                // If string digit exists use that
                (_, Some(f_mat)) => *num_map.get(f_mat.as_str()).unwrap(),
                // Default to char digit
                (Some((_, d)), _) => d,
                _ => unreachable!(),
            },
        );
        // dbg!(&line, &first, &last);

        acc + first * 10 + last
    })
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
        assert_eq!(d01p2(P2_INPUT.to_string()), P2_ANSWER)
    }
}
