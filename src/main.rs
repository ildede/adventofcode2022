use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use clap::Parser;
use colored::Colorize;

mod solvers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to solve
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    /// Which part (A or B) to solve
    #[arg(short, long)]
    part: String,
}

fn main() {
    let args: Args = Args::parse();
    let part = Part::from_str(args.part.as_str()).expect("invalid argument 'part', use A or B");
    println!("Solve day {} part {:?}!", args.day, part);
    let result = match args.day {
        1 => {
            format!(
                "{}: {}",
                format!("SOLUTION").green(),
                solvers::day1::solve(part, read_lines("input/day1"))
            )
        },
        2 => {
            format!(
                "{}: {}",
                format!("SOLUTION").green(),
                solvers::day2::solve(part, read_lines("input/day2"))
            )
        },
        3 => {
            format!(
                "{}: {}",
                format!("SOLUTION").green(),
                solvers::day3::solve(part, read_lines("input/day3"))
            )
        },
        4 => {
            format!(
                "{}: {}",
                format!("SOLUTION").green(),
                solvers::day4::solve(part, read_lines("input/day4"))
            )
        },
        5 => {
            format!(
                "{}: {}",
                format!("SOLUTION").green(),
/*
    [C]         [Q]         [V]
    [D]         [D] [S]     [M] [Z]
    [G]     [P] [W] [M]     [C] [G]
    [F]     [Z] [C] [D] [P] [S] [W]
[P] [L]     [C] [V] [W] [W] [H] [L]
[G] [B] [V] [R] [L] [N] [G] [P] [F]
[R] [T] [S] [S] [S] [T] [D] [L] [P]
[N] [J] [M] [L] [P] [C] [H] [Z] [R]
 1   2   3   4   5   6   7   8   9
*/
                solvers::day5::solve(part, vec![
                    vec!['N','R','G','P'],
                    vec!['J','T','B','L','F','G','D','C'],
                    vec!['M','S','V'],
                    vec!['L','S','R','C','Z','P'],
                    vec!['P','S','L','V','C','W','D','Q'],
                    vec!['C','T','N','W','D','M','S'],
                    vec!['H','D','G','W','P'],
                    vec!['Z','L','P','H','S','C','M','V'],
                    vec!['R','P','F','L','W','G','Z'],
                ],read_lines("input/day5"))
            )
        },
        6 => {
            format!(
                "{}: {}",
                format!("SOLUTION").green(),
                solvers::day6::solve(part, read_lines("input/day6").pop().unwrap())
            )
        },
        _ => format!("{}: Solver for day {} not implemented", format!("ERROR").red(), args.day)
    };
    println!("{}", result);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(_) => println!("Unreadable line? 🤔")
        }
    }
    lines
}

#[derive(Debug)]
pub enum Part {
    A,
    B
}
impl FromStr for Part {
    type Err = ();
    fn from_str(input: &str) -> Result<Part, Self::Err> {
        match input {
            "A"  => Ok(Part::A),
            "B"  => Ok(Part::B),
            _      => Err(()),
        }
    }
}