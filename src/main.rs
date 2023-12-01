use d01::*;

fn main() {
    let d01_input = include_str!("../input/d01");

    println!("Day 01 Part 1: {}", d01p1(d01_input.to_string()));
    println!("Day 01 Part 2: {}", d01p2(d01_input.to_string()));
}
