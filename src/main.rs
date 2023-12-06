use d01::*;
use d02::*;
use d03::*;
use d04::*;
use d05::*;
use d06::*;

fn main() {
    let d01_input = include_str!("../input/d01");
    println!("Day 01 Part 1: {}", d01p1(d01_input.to_string()));
    println!("Day 01 Part 2: {}", d01p2(d01_input.to_string()));

    let d02_input = include_str!("../input/d02");
    println!("Day 02 Part 1: {}", d02p1(d02_input.to_string()));
    println!("Day 02 Part 2: {}", d02p2(d02_input.to_string()));

    let d03_input = include_str!("../input/d03");
    println!("Day 03 Part 1: {}", d03p1(d03_input.to_string()));
    println!("Day 03 Part 2: {}", d03p2(d03_input.to_string()));

    let d04_input = include_str!("../input/d04");
    println!("Day 04 Part 1: {}", d04p1(d04_input.to_string()));
    // println!("Day 04 Part 2: {}", d04p2(d04_input.to_string()));

    let d05_input = include_str!("../input/d05");
    println!("Day 05 Part 1: {}", d05p1(d05_input.to_string()));
    // println!("Day 05 Part 2: {}", d05p2(d05_input.to_string()));

    let d06_input = include_str!("../input/d06");
    println!("Day 06 Part 1: {}", d06p1(d06_input.to_string()));
    println!("Day 06 Part 2: {}", d06p2(d06_input.to_string()));
}
