use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, char, digit1, multispace1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::{pair, separated_pair, tuple},
    IResult,
};

pub fn d05p1(input: String) -> u32 {
    let (seeds, maps) = parse_input(&input).unwrap().1;

    seeds
        .iter()
        .map(|s| maps.iter().fold(s.clone(), |acc, map| map.convert(acc)))
        .min()
        .unwrap() as u32
}

pub fn d05p2(input: String) -> u32 {
    todo!()
}

// -------------------

fn parse_input(input: &str) -> IResult<&str, (Vec<usize>, Vec<Map>)> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = multispace1(input)?;
    Ok((input, (seeds, parse_maps(input))))
}

fn parse_maps(input: &str) -> Vec<Map> {
    separated_list1(multispace1, parse_map)(input).unwrap().1
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, (to, from)) = parse_to_from(input)?;
    let (input, _) = pair(multispace1, tag("map:\n"))(input)?;
    let (input, ranges) = parse_map_ranges(input)?;

    Ok((input, Map { to, from, ranges }))
}

fn parse_to_from(input: &str) -> IResult<&str, (Category, Category)> {
    separated_pair(
        parse_category,
        tuple((char('-'), tag("to"), char('-'))),
        parse_category,
    )(input)
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    map_res(alpha0, |s| Category::from_str(s))(input)
}

fn parse_map_ranges(input: &str) -> IResult<&str, Vec<MapRange>> {
    separated_list1(newline, parse_map_range)(input)
}

fn parse_map_range(input: &str) -> IResult<&str, MapRange> {
    let (input, dest_start) = parse_num(input)?;
    let (input, _) = multispace1(input)?;
    let (input, src_start) = parse_num(input)?;
    let (input, _) = multispace1(input)?;
    let (input, len) = parse_num(input)?;

    Ok((
        input,
        MapRange {
            dest_start,
            src_start,
            len,
        },
    ))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tuple((tag("seeds"), char(':'), multispace1))(input)?;
    parse_num_list(&input)
}

fn parse_num_list(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(multispace1, parse_num)(input)
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

// -------------------

#[derive(Debug, PartialEq, Clone)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    to: Category,
    from: Category,
    ranges: Vec<MapRange>,
}

impl Map {
    fn convert(&self, input: usize) -> usize {
        match self
            .ranges
            .iter()
            .filter(|r| r.src_start <= input && input <= (r.src_start + r.len))
            .take(1)
            .next()
        {
            Some(range) => range.dest_start + (input - range.src_start),
            _ => input,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct MapRange {
    dest_start: usize,
    src_start: usize,
    len: usize,
}

#[cfg(test)]
mod d05_tests {
    use super::*;

    const P1_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
    const P2_INPUT: &str = "";
    const P1_ANSWER: u32 = 35;
    const P2_ANSWER: u32 = 0;

    #[test]
    fn src_to_dest_convert_works() {
        let (seeds, maps) = parse_input(P1_INPUT).unwrap().1;
        let answer = maps[0].convert(seeds[0]);
        assert_eq!(answer, 81);
    }

    #[test]
    fn parse_map_range_works() {
        assert_eq!(
            parse_map_range("45 77 23\n").unwrap().1,
            MapRange {
                dest_start: 45,
                src_start: 77,
                len: 23
            }
        );
    }

    #[test]
    fn parse_map_ranges_works() {
        assert_eq!(
            parse_map_ranges("45 77 23\n3 43 2").unwrap().1,
            vec![
                MapRange {
                    dest_start: 45,
                    src_start: 77,
                    len: 23
                },
                MapRange {
                    dest_start: 3,
                    src_start: 43,
                    len: 2
                }
            ]
        );
    }

    #[test]
    fn parse_category_works() {
        assert_eq!(parse_category("soil").unwrap().1, Category::Soil);
    }

    #[test]
    fn parse_to_from_works() {
        assert_eq!(
            parse_to_from("seed-to-soil").unwrap().1,
            (Category::Seed, Category::Soil)
        );
    }

    #[test]
    fn parse_map_works() {
        assert_eq!(
            parse_map("soil-to-fertilizer map:\n23 43 1\n42 2 20")
                .unwrap()
                .1,
            Map {
                to: Category::Soil,
                from: Category::Fertilizer,
                ranges: vec![
                    MapRange {
                        dest_start: 23,
                        src_start: 43,
                        len: 1
                    },
                    MapRange {
                        dest_start: 42,
                        src_start: 2,
                        len: 20,
                    }
                ],
            }
        )
    }

    #[test]
    fn parse_maps_works() {
        assert_eq!(
            parse_maps("soil-to-fertilizer map:\n23 43 1\n42 2 20\n\nfertilizer-to-water map:\n23 43 1\n42 2 20"),
            vec![
                Map {
                to: Category::Soil,
                from: Category::Fertilizer,
                ranges: vec![
                    MapRange {
                        dest_start: 23,
                        src_start: 43,
                        len: 1
                    },
                    MapRange {
                        dest_start: 42,
                        src_start: 2,
                        len: 20,
                    }
                ],
            }, Map {
                to: Category::Fertilizer,
                from: Category::Water,
                ranges: vec![
                    MapRange {
                        dest_start: 23,
                        src_start: 43,
                        len: 1
                    },
                    MapRange {
                        dest_start: 42,
                        src_start: 2,
                        len: 20,
                    }
                ],
            }]
        )
    }

    #[test]
    fn passes_part_one() {
        assert_eq!(d05p1(P1_INPUT.to_string()), P1_ANSWER)
    }

    #[test]
    fn passes_part_two() {
        assert_eq!(d05p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
