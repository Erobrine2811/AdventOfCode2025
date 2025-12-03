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
    let mut count = 0;
    'outer: for line in input {
        let lineLen = line.len();
        let substring = &line[..(lineLen as usize)-1];

        let mut index = 0;
        let mut largest = 0;
        let mut largestIndex = 0;

        'inner: for char in substring.chars() {
            let asNum  = char.to_digit(10).unwrap();
            if asNum > largest {
                largest = asNum;
                largestIndex = index;
            }

            if largest == 9 {
                break 'inner;
            }

            index+=1;
        }

        let newSubstring = &line[largestIndex+1..];

        let mut secondLargest = 0;
        'inner: for char in newSubstring.chars() {
            let asNum  = char.to_digit(10).unwrap();

            if asNum > secondLargest {
                secondLargest = asNum;
            }
        }

        println!("{}", (10 * largest) + secondLargest);
        count = count + (10 * largest) + secondLargest;

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
