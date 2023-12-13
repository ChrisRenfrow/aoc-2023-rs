pub fn d13p1(input: String) -> u32 {
    todo!()
}

pub fn d13p2(_input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod d13_tests {
    use super::*;

    const P1_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;
    const P1_ANSWER: u32 = 405;
    const P2_ANSWER: u32 = 0;

    #[test]
    fn passes_part_one() {
        assert_eq!(d13p1(P1_INPUT.to_string()), P1_ANSWER);
    }

    #[test]
    fn passes_part_two() {
        assert_eq!(d13p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
