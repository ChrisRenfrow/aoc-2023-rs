use regex::Regex;

// Part numbers have one symbol (*, #, +, $) adjacent or diagonal to them.
// What is the sum of all of the part numbers in the engine schematic?
pub fn d03p1(input: String) -> u32 {
    let (symbols, numbers) = parse_input(input);

    symbols.iter().fold(0, |sum, symbol| {
        sum + numbers
            .iter()
            .filter(|n| {
                (n.pos.x..n.pos.x + n.len)
                    .any(|i| is_adjacent(&(Coord { x: i, y: n.pos.y }), &symbol.pos))
            })
            .fold(0, |sum, part| sum + part.num)
    })
}

// This time, you need to find the gear ratio of every gear
// and add them all up so that the engineer can figure out
// which gear needs to be replaced.
pub fn d03p2(input: String) -> u32 {
    todo!()
}

fn parse_input(input: String) -> (Vec<Symbol>, Vec<PartNum>) {
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<PartNum> = vec![];

    let sym_re = Regex::new(r"[^0-9\.]").unwrap();
    let num_re = Regex::new(r"[0-9]+").unwrap();

    input.lines().enumerate().for_each(|(y, line)| {
        let mut syms: Vec<Symbol> = sym_re
            .find_iter(line)
            .map(|m| Symbol {
                chr: m.as_str().chars().next().unwrap(),
                pos: Coord { x: m.start(), y },
            })
            .collect();
        let mut nums: Vec<PartNum> = num_re
            .find_iter(line)
            .map(|m| PartNum {
                num: m.as_str().parse::<u32>().unwrap(),
                pos: Coord { x: m.start(), y },
                len: m.len(),
            })
            .collect();

        symbols.append(&mut syms);
        numbers.append(&mut nums);
    });

    (symbols, numbers)
}

fn is_adjacent(a: &Coord, b: &Coord) -> bool {
    let x_diff = (a.x as isize - b.x as isize).abs();
    let y_diff = (a.y as isize - b.y as isize).abs();

    x_diff <= 1 && y_diff <= 1 && (x_diff == 0 || y_diff == 0 || x_diff == y_diff)
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PartNum {
    num: u32,
    pos: Coord,
    len: usize,
}

#[derive(Debug)]
struct Symbol {
    chr: char,
    pos: Coord,
}

#[cfg(test)]
mod d03_tests {
    use super::*;

    const P1_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    const P2_INPUT: &str = r#"{{EXAMPLE_2}}"#;
    const P1_ANSWER: u32 = 4361;
    const P2_ANSWER: u32 = 467835;

    #[test]
    fn passes_part_one() {
        assert_eq!(d03p1(P1_INPUT.to_string()), P1_ANSWER)
    }

    #[test]
    fn passes_part_two() {
        // NOTE: Future-me, make sure you're using the right input before you go throwing `dbg!()` calls everywhere :)
        assert_eq!(d03p2(P1_INPUT.to_string()), P2_ANSWER)
    }
}
