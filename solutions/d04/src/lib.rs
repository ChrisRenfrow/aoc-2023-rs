use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{pair, separated_pair, tuple},
    IResult,
};

pub fn d04p1(input: String) -> u32 {
    let cards = parse_cards(input);
    cards.iter().fold(0, |total, card| {
        total
            + card.winning_nums.iter().fold(0, |acc, win| {
                if card.scratch_nums.contains(win) {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                } else {
                    acc
                }
            })
    })
}

pub fn d04p2(input: String) -> u32 {
    todo!()
}

fn parse_cards(input: String) -> Vec<Card> {
    input
        .lines()
        .map(|line| parse_card(&line).unwrap().1)
        .collect()
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = pair(tag("Card"), multispace1)(input)?;
    let (input, id) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = pair(tag(":"), multispace1)(input)?;
    let (input, (winning_nums, scratch_nums)) = parse_list_pair(input)?;

    Ok((
        input,
        Card {
            id,
            winning_nums,
            scratch_nums,
        },
    ))
}

fn parse_list_pair(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(
        parse_list,
        tuple((multispace1, char('|'), multispace1)),
        parse_list,
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(multispace1, parse_u32)(input)
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

struct Card {
    id: u32,
    winning_nums: Vec<u32>,
    scratch_nums: Vec<u32>,
}

#[cfg(test)]
mod d04_tests {
    use super::*;

    const P1_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    // In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17)
    // and eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers
    // you have, four of them (48, 83, 17, and 86) are winning numbers! That means
    // card 1 is worth 8 points (1 for the first match, then doubled three times for
    // each of the three matches after the first).

    // Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
    // Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
    // Card 4 has one winning number (84), so it is worth 1 point.
    // Card 5 has no winning numbers, so it is worth no points.
    // Card 6 has no winning numbers, so it is worth no points.
    const P2_INPUT: &str = "";
    const P1_ANSWER: u32 = 13;
    const P2_ANSWER: u32 = 0;

    #[test]
    fn parse_num_works() {
        assert_eq!(parse_u32("45"), Ok(("", 45)));
    }

    #[test]
    fn parse_list_works() {
        assert_eq!(parse_list("43 54 83  3"), Ok(("", vec![43, 54, 83, 3])));
    }

    #[test]
    fn parse_list_pair_works() {
        assert_eq!(
            parse_list_pair("43 54 83  3 | 73  9 2"),
            Ok(("", (vec![43, 54, 83, 3], vec![73, 9, 2])))
        );
    }

    #[test]
    fn passes_part_one() {
        assert_eq!(d04p1(P1_INPUT.to_string()), P1_ANSWER)
    }

    #[test]
    fn passes_part_two() {
        assert_eq!(d04p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
