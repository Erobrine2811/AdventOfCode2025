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
    let mut count : u64 = 0;
    let range = 12;
    for line in input {
        let mut nums: Vec<u64> = Vec::new();
        let lineLen = line.len();


        let mut workingSubstring = &line[..(lineLen as usize)-(range - 1)];

        let mut id = 0;

        for i in 1..=range {
            let mut index = 0;
            let mut largest = 0;
            let mut largestIndex = 0;

            'inner: for char in workingSubstring.chars() {
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

            id = id + largestIndex + 1;

            nums.push(largest as u64);

            if i != range {
                workingSubstring = &line[id..(lineLen as usize) - (range - (i+1)  as usize)];
            }
        }


        let mut index = 0;

        let mut sum = 0;

        for num in &nums {
            index += 1;

            let mut multiplier = 10_u64.pow((range - index).try_into().unwrap());
            sum = sum + num * multiplier;
        }
        count = count + sum;

        nums.clear();

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
