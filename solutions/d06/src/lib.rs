use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, digit1, multispace0, multispace1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, tuple},
    IResult,
};

pub fn d06p1(input: String) -> u32 {
    let (time, distance) = parse_input(&input).unwrap().1;

    time.iter()
        .cloned()
        .zip(distance.iter().cloned())
        .fold(1, |acc, (t, d)| acc * n_ways_to_win(t, d)) as u32
}

pub fn d06p2(input: String) -> u32 {
    let (time, distance) = parse_input(&input).unwrap().1;
    let (time, distance) = (join_nums(time), join_nums(distance));
    n_ways_to_win(time, distance) as u32
}

fn n_ways_to_win(time: usize, dist: usize) -> usize {
    (1..time).filter(|n| n * (time - n) > dist).count()
}

fn join_nums(nums: Vec<usize>) -> usize {
    nums.iter()
        .cloned()
        .fold("".to_string(), |mut acc, n| {
            acc.push_str(&n.to_string());
            acc
        })
        .parse()
        .unwrap()
}

fn parse_input(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    let (input, _) = pair(tag("Time:"), multispace1)(input)?;
    let (input, time) = separated_list1(multispace1, parse_num)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = pair(tag("Distance:"), multispace1)(input)?;
    let (input, dist) = separated_list1(multispace1, parse_num)(input)?;
    Ok((input, (time, dist)))
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

#[cfg(test)]
mod d06_tests {
    use super::*;

    const P1_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
    const P2_INPUT: &str = "";
    const P1_ANSWER: u32 = 288;
    const P2_ANSWER: u32 = 71503;

    #[test]
    fn parse_input_works() {
        assert_eq!(
            parse_input(P1_INPUT).unwrap().1,
            (vec![7, 15, 30], vec![9, 40, 200])
        );
    }

    #[test]
    fn passes_part_one() {
        assert_eq!(d06p1(P1_INPUT.to_string()), P1_ANSWER)
    }

    #[test]
    fn passes_part_two() {
        assert_eq!(d06p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
