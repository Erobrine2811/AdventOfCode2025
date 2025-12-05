use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect()
}

fn solve(input: &[String]) -> String {
    let mut count = 0;
    let mut readingIds = false;

    let mut ids = Vec::new();
    let mut ranges = Vec::new();

    for line in input {
        if line == "" {
            readingIds = true;
            continue;
        }

        if readingIds {
            let id: i64 = line.parse::<i64>().unwrap();

            ids.push(id);
        } else {
            let Some((from, to)) = line.split_once("-") else { continue; };
            ranges.push(((from.parse::<i64>().unwrap()), (to.parse::<i64>().unwrap())));
        }
    }

    for id in &ids {
        'inner: for range in &ranges {
            let from = range.0;
            let to = range.1;
            if id >= &from  && id <= &to {
                count += 1;
                break 'inner;
            }
        }
    }

    return count.to_string()
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
