pub fn d04p1(input: String) -> u32 {
    todo!()
}

pub fn d04p2(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod d04_tests {
    use super::*;

    const P1_INPUT: &str = r#""#;
    const P2_INPUT: &str = "";
    const P1_ANSWER: u32 = 0;
    const P2_ANSWER: u32 = 0;

    #[test]
    fn passes_part_one() {
        assert_eq!(d04p1(P1_INPUT.to_string()), P1_ANSWER)
    }

    #[test]
    fn passes_part_two() {
        assert_eq!(d04p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
