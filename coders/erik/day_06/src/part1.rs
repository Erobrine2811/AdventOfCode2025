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
    let mut problems: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    let mut first = true;

    let numOfLines = input.len();

    let mut lineIndex = 0;
    for line in input {
        let rawNums = line.split(" ");

        if lineIndex == numOfLines-1 { 
            for n in rawNums {
                if n == "" { continue; }

                operators.push(n.parse::<String>().unwrap());
            }

            break;
        }

        let mut numsRow: Vec<i64> = Vec::new();


        for n in rawNums {
            if n == "" { continue; }
            let asNum = n.parse::<i64>().unwrap();
            numsRow.push(asNum);
        }

        let mut i = 0;
        for n in numsRow {
            if first {
                problems.push(Vec::new());
            }
            problems[i].push(n);
            i+=1;
        }

        first = false;
        lineIndex+=1;
    }

    let mut count = 0;

    let mut i = 0;
    for problem in problems {
        let operator = &operators[i];

        let mut s = 0;
        if operator == "*" {
            s = 1;
        }
        for num in problem {
            if operator == "*" {
                s = s * num;
            } else {
                s = s + num;
            }
        }
        count += s;

        i+=1;
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
