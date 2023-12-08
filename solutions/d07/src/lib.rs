pub fn d07p1(input: String) -> u32 {
    let hands = parse_input(&input);

    hands
        .iter()
        .for_each(|(hand, bid)| println!("Hand: {:#?}\nBid: {}", hand, bid));
    todo!()
}

pub fn d07p2(input: String) -> u32 {
    todo!()
}

fn parse_input(input: &str) -> Vec<(Vec<Card>, u32)> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(input: &str) -> (Vec<Card>, u32) {
    let mut parts = input.split(" ");
    let hand_str = parts.next().unwrap();
    let bid_str = parts.next().unwrap();
    (parse_hand(hand_str), parse_bid(bid_str))
}

fn parse_bid(input: &str) -> u32 {
    input.parse().unwrap()
}

fn parse_hand(input: &str) -> Vec<Card> {
    input
        .chars()
        .map(|c| match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            _ => Card::N(c.to_digit(10).unwrap() as u32),
        })
        .collect()
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u32)]
enum Card {
    N(u32),
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

#[cfg(test)]
mod d07_tests {
    use super::*;

    const P1_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;
    const P2_INPUT: &str = "";
    const P1_ANSWER: u32 = 6440;
    const P2_ANSWER: u32 = 0;

    #[test]
    fn parse_hand_works() {
        assert_eq!(
            parse_hand("AKQJT98765432"),
            vec![
                Card::A,
                Card::K,
                Card::Q,
                Card::J,
                Card::T,
                Card::N(9),
                Card::N(8),
                Card::N(7),
                Card::N(6),
                Card::N(5),
                Card::N(4),
                Card::N(3),
                Card::N(2),
            ]
        )
    }

    #[test]
    fn passes_part_one() {
        assert_eq!(d07p1(P1_INPUT.to_string()), P1_ANSWER)
    }

    #[test]
    fn passes_part_two() {
        assert_eq!(d07p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
