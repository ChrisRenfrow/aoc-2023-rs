pub fn d03p1(input: String) -> u32 {
    todo!()
}

pub fn d03p2(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod d03_tests {
    use super::*;

    const P1_INPUT: &str = r#"{{EXAMPLE_1}}"#;
    const P2_INPUT: &str = r#"{{EXAMPLE_2}}"#;
    const P1_ANSWER: u32 = 0;
    const P2_ANSWER: u32 = 0;

    #[test]
    fn passes_part_one() {
        assert_eq!(d03p1(P1_INPUT.to_string()), P1_ANSWER)
    }

    #[test]
    fn passes_part_two() {
        // NOTE: Future-me, make sure you're using the right input before you go throwing `dbg!()` calls everywhere :)
        assert_eq!(d03p2(P2_INPUT.to_string()), P2_ANSWER)
    }
}
