use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect()
}

fn solve(input: &[String]) -> String {
    let mut dial = 50;
    let mut count = 0;

    for line in input {
        let direction = &line[0..1];

        let mut num : i32 = line[1..].parse().unwrap();

        let numOfFullRotations = ((num as f64)/100.0).floor().abs() as i32;
        count += numOfFullRotations;

        num = num % 100;

        if ( direction == "L") {
            dial = dial - num;
            if dial < 0 && num != dial.abs() { count += 1 }
        }
        else {
            dial = dial + num;
            if dial > 100 && num != 100 - dial { count += 1 }
        }

        dial = dial % 100;


        if dial == 0 {
            count += 1;
        }

        if dial < 0 {
            dial = 100 + dial
        }

    }

    return count.to_string();
}

fn main() {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 { &args[1] } else { "input.txt" };

    if let Ok(lines) = read_lines(input_file) {
        let solution = solve(&lines);
        let duration = start_time.elapsed();
        println!("Solution: {}", solution);
        println!("Completed in {:.4} seconds", duration.as_secs_f64());
    } else {
        eprintln!("Error reading input file");
    }
}
