use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0},
    combinator::map,
    multi::many1,
    sequence::{pair, separated_pair},
    IResult,
};

pub fn d08p1(input: String) -> u32 {
    let (directions, nodes) = parse_input(&input);
    let mut directions = VecDeque::from(directions);
    let mut key = "AAA".to_string();
    let mut count = 0;

    while key != "ZZZ" {
        let node = nodes.get(&key).unwrap();
        let direction = directions.pop_front().unwrap();
        match direction {
            Dir::Left => key = node.0.clone(),
            Dir::Right => key = node.1.clone(),
        }
        directions.push_back(direction);
        count += 1;
    }

    count
}

pub fn d08p2(_input: String) -> u32 {
    todo!()
}

fn parse_input(input: &str) -> (Vec<Dir>, HashMap<String, (String, String)>) {
    let (input, directions) = parse_directions(input).unwrap();
    let nodes = parse_nodes(input);
    (directions, nodes)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Dir>> {
    many1(alt((
        map(char('L'), |_| Dir::Left),
        map(char('R'), |_| Dir::Right),
    )))(input)
}

fn parse_nodes(input: &str) -> HashMap<String, (String, String)> {
    HashMap::from_iter(input.trim().lines().map(|l| parse_node(l).unwrap().1))
}

// AAA = (BBB, CCC)
fn parse_node(input: &str) -> IResult<&str, (String, (String, String))> {
    let (input, key) = alpha1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, val) = separated_pair(alpha1, tag(", "), alpha1)(input)?;
    let (input, _) = pair(tag(")"), multispace0)(input)?;
    Ok((
        input,
        (key.to_string(), (val.0.to_string(), val.1.to_string())),
    ))
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Dir {
    Left,
    Right,
}

#[cfg(test)]
mod d08_tests {
    use super::*;

    const P1_INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
    const P1_ALT_INPUT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;
    const P1_ANSWER: u32 = 2;
    const P1_ALT_ANSWER: u32 = 6;
    const P2_ANSWER: u32 = 0;

    #[test]
    fn parse_nodes_works() {
        let map = HashMap::from([
            ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string())),
            ("BBB".to_string(), ("DDD".to_string(), "EEE".to_string())),
        ]);
        let input = r#"AAA = (BBB, CCC)
BBB = (DDD, EEE)"#;
        assert_eq!(parse_nodes(input), map);
    }

    #[test]
    fn parse_node_works() {
        assert_eq!(
            parse_node("AAA = (BBB, CCC)").unwrap().1,
            ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()))
        );
    }

    #[test]
    fn passes_part_one() {
        assert_eq!(d08p1(P1_INPUT.to_string()), P1_ANSWER);
        assert_eq!(d08p1(P1_ALT_INPUT.to_string()), P1_ALT_ANSWER);
    }

    #[test]
    fn passes_part_two() {
        assert_eq!(d08p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
