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

    let mut ranges = Vec::new();

    for line in input {
        if line == "" {
            break;
        }

        let Some((from, to)) = line.split_once("-") else { continue; };
        ranges.push(((from.parse::<i64>().unwrap()), (to.parse::<i64>().unwrap())));
    }

    let mut smallest = i64::MAX;
    let mut biggest = 0;

    for range in &ranges {
        let from = range.0;
        let to = range.1;

        if from < smallest {
            smallest = from;
        }
        if to > biggest {
            biggest = to;
        }
    }
    let mut smallestRange: &(i64, i64) = &(0i64, 0i64);

    for range in &ranges {
        let from = range.0;
        let to = range.1;

        if from == smallest {
            smallestRange = range;
            break;
        }
    }


    let mut pointerFrom = smallestRange.0;
    let mut pointerTo = smallestRange.1;

    while true {

        'inner: while true {
            let startTo = pointerTo;

            for range in &ranges {
                let from = range.0;
                let to = range.1;
                if pointerTo >= from && pointerFrom <= from && to > pointerTo {
                    pointerTo = to;
                }
            }

            if startTo == pointerTo {
                break 'inner;
            }
        }


        let mut smallest = i64::MAX;
        let mut smallestTo = 0;

        for range in &ranges {
            let from = range.0;
            let to = range.1;

            if from < smallest && from > pointerFrom && pointerTo < to {
                smallest = from;
                smallestTo = to;
            }
        }

        count = count + (pointerTo - pointerFrom + 1);

        if smallest == i64::MAX {
            break;
        }

        pointerFrom = smallest;
        pointerTo = smallestTo;
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
