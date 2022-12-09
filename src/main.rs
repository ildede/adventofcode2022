use std::fs::File;
use std::io::{BufRead, BufReader};
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
    let args = Args::parse();

    println!("Solve day {} part {}!", args.day, args.part);
    match args.day {
        1 => {
            let lines = read_lines("input/day1");
            let result = solvers::day1::solve(lines);
            println!("{}: {}", format!("SOLUTION").green(), result);
        },
        _ => println!("{}: Solver for day {} not implemented", format!("ERROR").red(), args.day)
    }
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