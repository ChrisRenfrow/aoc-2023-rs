use std::fs;

use clap::Parser;
use colored::Colorize;

use d01::*;
use d02::*;
use d03::*;
use d04::*;
use d05::*;
use d06::*;

fn main() {
    let cli = Cli::parse();

    let solutions: Vec<Day> = vec![
        Day::new(1, Some(&d01p1), Some(&d01p2)),
        Day::new(2, Some(&d02p1), Some(&d02p2)),
        Day::new(3, Some(&d03p1), Some(&d03p2)),
        Day::new(4, Some(&d04p1), None),
        Day::new(5, Some(&d05p1), None),
        Day::new(6, Some(&d06p1), Some(&d06p2)),
    ];

    match cli.day {
        Some(d) => solutions[d - 1].run(),
        None => solutions.iter().for_each(|d| d.run()),
    };
}

type SolutionFn = dyn Fn(String) -> u32;

#[derive(Parser)]
struct Cli {
    /// A specific day to run
    #[clap(short, long)]
    day: Option<usize>,
}

#[derive(Clone)]
struct Day<'a> {
    num: usize,
    p1: Option<&'a SolutionFn>,
    p2: Option<&'a SolutionFn>,
    input: String,
}

impl<'a> Day<'a> {
    fn new(num: usize, p1: Option<&'a SolutionFn>, p2: Option<&'a SolutionFn>) -> Self {
        Self {
            num,
            p1,
            p2,
            input: day_input(num),
        }
    }

    fn run(&self) {
        println!(
            "==( {} )===========",
            format!("Day {:>2}", self.num).yellow()
        );
        self.run_part(1);
        self.run_part(2);
    }

    fn run_part(&self, part_num: usize) {
        let part = match part_num {
            1 => self.p1,
            2 => self.p2,
            _ => panic!("There are only 2 parts to any solution"),
        };
        match part {
            Some(part) => println!(
                "  Part {}: {}",
                part_num,
                part(self.input.clone()).to_string().green()
            ),
            None => println!("  Part {}: {}", part_num, "SKIPPED".red()),
        }
    }
}

fn day_input(day: usize) -> String {
    fs::read_to_string(format!("input/d{:02}", day))
        .expect("Should have been able to read input file")
}
