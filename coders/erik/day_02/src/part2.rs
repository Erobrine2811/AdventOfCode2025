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
    let mut ranges = Vec::new();
    for line in input {
        let input_by_comma = line.split(",");

        for range in input_by_comma {
            let Some((part1, part2)) = range.split_once("-") else { continue;};
            ranges.push([part1.parse::<i64>().unwrap(), part2.parse::<i64>().unwrap()]);
        }
    }

    let mut count: i64 = 0;

    for range in ranges {
        for num in range[0]..=range[1] {
            if check_invalidity(num) {
                count += num;
            }
        }
    }

    return count.to_string();
}


fn check_invalidity(num: i64) -> bool {
    let numAsStr  = num.to_string();
    let numAsStrLen  = numAsStr.len();
    'outer: for i in 1..=((numAsStrLen as i32) / 2) {
        if (numAsStrLen as i32) % i != 0 { continue; }

        let substring = &numAsStr[..(i as usize)];

        let mut failed = false;

        'inner: for x in 1..((numAsStrLen as i32) / i) {
            let newSubstring = &numAsStr[((i as usize)* (x as usize))..(((x+1) as usize)*(i as usize))];

            if substring != newSubstring {
                failed = true;
                break 'inner;
            }
        }

        if !failed {
            return true;
        }
    }
    return false;
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
