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

fn hasFourNeighbours(x: usize, y: usize, grid: Vec<String>) -> bool {
    let char = grid[y].as_bytes()[x] as char;

    if char != '@' { return false; }

    let height = grid.len();
    let width = grid[0].len();

    let mut cnt = 0;

    // Up
    if y > 0 {
        if grid[y-1].as_bytes()[x] as char != '.' { cnt += 1; }
    }
    // Left
    if x > 0 {
        if grid[y].as_bytes()[x-1] as char != '.' { cnt += 1; }
    }
    // Left Up
    if x > 0 && y > 0 {
        if grid[y-1].as_bytes()[x-1] as char != '.' { cnt += 1; }
    }

    // Right Down
    if x < width - 1 && y < height - 1 {
        if grid[y+1].as_bytes()[x+1] as char != '.' { cnt += 1; }
    }
    // Right
    if x < width - 1 {
        if grid[y].as_bytes()[x+1] as char != '.' { cnt += 1; }
    }
    // Down
    if y < height - 1 {
        if grid[y+1].as_bytes()[x] as char != '.' { cnt += 1; }
    }

    // Left Down
    if y < height - 1 && x > 0 {
        if grid[y+1].as_bytes()[x-1] as char != '.' { cnt += 1; }
    }

    // Right Up
    if y > 0 && x < width - 1 {
        if grid[y-1].as_bytes()[x+1] as char != '.' { cnt += 1; }
    }

    return cnt < 4;
}

fn solve(input: &[String]) -> String {
    let mut grid: Vec<String> = Vec::new();
    for line in input{
        grid.push(line.to_string());
    }

    let height = grid.len();
    let width = grid[0].len();
    
    let mut count = 0;

    while (true) {
        for y in 0..height {
            for x in 0..width {
                if hasFourNeighbours(x, y, grid.clone()) {
                    count += 1;
                    grid[y].replace_range(x..x+1, "x");
                }
            }
        }
        let mut finished = true;

        'outer: for y in 0..height {
            for x in 0..width {
                let char = grid[y].as_bytes()[x] as char;
                if char == 'x' {
                    finished = false;
                    break 'outer;
                }
            }
        }

        if finished {
            break;
        }


        // Clear
        for y in 0..height {
            for x in 0..width {
                grid[y] = grid[y].replace("x", ".");
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
