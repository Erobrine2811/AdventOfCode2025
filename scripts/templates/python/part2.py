import sys
import time

def read_input(file_path):
    with open(file_path, "r") as f:
        return f.read().splitlines()

def solve(input_data):
    # Your solution for part 2 goes here
    return "solution for part 2"

if __name__ == "__main__":
    start_time = time.time()
    input_file = "input.txt"
    if len(sys.argv) > 1:
        input_file = sys.argv[1]
    
    input_data = read_input(input_file)
    solution = solve(input_data)
    
    end_time = time.time()
    
    print(f"Solution: {solution}")
    print(f"Completed in {end_time - start_time:.4f} seconds")
