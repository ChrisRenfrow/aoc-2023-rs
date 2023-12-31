// Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
// 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, char, digit1, multispace0},
    combinator::map_res,
    multi::separated_list0,
    sequence::{pair, preceded},
    IResult,
};

pub fn d02p1(input: String) -> u32 {
    // The number of cubes loaded in the bag
    let (red, green, blue) = (12, 13, 14);
    let games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line))
        .map(|game| game.unwrap().1)
        .collect();

    games.iter().fold(0, |sum, game| {
        if game.is_valid(red, green, blue) {
            sum + game.id
        } else {
            sum
        }
    })
}

pub fn d02p2(input: String) -> u32 {
    let games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line))
        .map(|game| game.unwrap().1)
        .collect();

    games.iter().fold(0, |acc, game| {
        let (min_red, min_green, min_blue) = game.get_min_cubes();
        acc + min_red * min_green * min_blue
    })
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<GameRound>,
}

impl Game {
    fn is_valid(&self, red: u32, green: u32, blue: u32) -> bool {
        for round in self.rounds[..].iter() {
            if !round.cube_counts.iter().all(|count| match count.color {
                Color::Red if count.count <= red => true,
                Color::Green if count.count <= green => true,
                Color::Blue if count.count <= blue => true,
                _ => false,
            }) {
                return false;
            }
        }
        return true;
    }

    fn get_min_cubes(&self) -> (u32, u32, u32) {
        self.rounds.iter().fold(
            (0, 0, 0),
            |(round_max_r, round_max_g, round_max_b), round| {
                round.cube_counts.iter().fold(
                    (round_max_r, round_max_g, round_max_b),
                    |(max_red, max_green, max_blue), count| match count.color {
                        Color::Red if count.count > max_red => (count.count, max_green, max_blue),
                        Color::Green if count.count > max_green => (max_red, count.count, max_blue),
                        Color::Blue if count.count > max_blue => (max_red, max_green, count.count),
                        _ => (max_red, max_green, max_blue),
                    },
                )
            },
        )
    }
}

#[derive(Debug)]
struct GameRound {
    cube_counts: Vec<CubeCount>,
}

#[derive(Debug)]
struct CubeCount {
    color: Color,
    count: u32,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 7 blue; 2 green
fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = pair(tag("Game"), multispace0)(input)?;
    let (input, id) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = pair(char(':'), multispace0)(input)?;
    let (input, rounds) = separated_list0(pair(char(';'), multispace0), parse_game_round)(input)?;

    Ok((input, Game { id, rounds }))
}

// 3 blue, 4 red
fn parse_game_round(input: &str) -> IResult<&str, GameRound> {
    let (input, _) = multispace0(input)?;
    let (input, cube_counts) =
        separated_list0(pair(char(','), multispace0), parse_cube_count)(input)?;

    Ok((input, GameRound { cube_counts }))
}

// 3 blue
fn parse_cube_count(input: &str) -> IResult<&str, CubeCount> {
    let (input, count) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = multispace0(input)?;
    let (input, color) = map_res(alpha0, |s: &str| Color::from_str(s.trim()))(input)?;

    Ok((input, CubeCount { color, count }))
}

#[cfg(test)]
mod d02_tests {
    use super::*;

    const P1_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    const P2_INPUT: &str = "";
    const P1_ANSWER: u32 = 8;
    const P2_ANSWER: u32 = 2286;

    #[test]
    fn passes_part_one() {
        assert_eq!(d02p1(P1_INPUT.to_string()), P1_ANSWER)
    }

    #[test]
    fn passes_part_two() {
        assert_eq!(d02p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
