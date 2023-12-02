use d01::*;
use d02::*;

fn main() {
    let d01_input = include_str!("../input/d01");
    let d02_input = include_str!("../input/d02");

    println!("Day 01 Part 1: {}", d01p1(d01_input.to_string()));
    println!("Day 01 Part 2: {}", d01p2(d01_input.to_string()));
    println!("Day 02 Part 1: {}", d02p1(d02_input.to_string()));
    println!("Day 02 Part 2: {}", d02p2(d02_input.to_string()));
}
