use std::env;
use std::fs::File;
use std::cmp::min;
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
    let mut problems: Vec<Vec<String>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    let mut first = true;

    let numOfLines = input.len();

    let mut lineIndex = 0;

    let mut problemsRaw: Vec<String> = Vec::new();


    for line in input {
        let rawNums = line.split(" ");

        if lineIndex == numOfLines-1 { 

            let mut s = "".to_string();
            let mut firstOperator = true;

            for n in rawNums {
                if n == "" { 
                    s.push_str(" ");
                    continue; 
                }
                if !firstOperator {
                    problems.push(Vec::new());
                    operators.push(s);
                    s = "".to_string();
                }
                s.push_str(n);

                firstOperator = false;
            }
            problems.push(Vec::new());
            operators.push(s);

            break;
        }

        problemsRaw.push(line.clone());

        lineIndex += 1;
    }

    let mut currProblem = 0;


    for line in problemsRaw {
        let mut pointer = 0;
        let lineLen = line.len();

        let mut problemI = 0;
        for op in &operators {
            let op = &operators[problemI];
            let op_len = op.len();

            problems[problemI].push(line[pointer..min(pointer+op_len, lineLen)].to_string());
            pointer = pointer + op_len+1;
            problemI += 1;
        }
    }

    let mut count: i64 = 0;

    let mut problemI = 0;
    for problem in problems {
        let len = problem[0].len();
        let op = operators[problemI].trim();

        let mut s : i64 = 0;
        if op == "*" {
            s = 1;
        }
        
        for index in 0..len {
            let mut num = "".to_string();
            for n in &problem {
                let ch : &str = &n[index..index+1];
                if ch == "" {continue;}

                num.push_str(ch);
            }
            if op == "*" {
                s = s * num.trim().parse::<i64>().unwrap();
            }
            else {
                s = s + num.trim().parse::<i64>().unwrap();
            }
        }

        count += s;
        problemI += 1;
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
