require 'time'

def read_input(file_path)
  File.readlines(file_path, chomp: true).filter { |line| !line.empty? }
end

def solve(input)
  # Your solution for part 1 goes here
  "solution for part 1"
end

start_time = Time.now
input_file = ARGV.length > 0 ? ARGV[0] : 'input.txt'
input = read_input(input_file)
solution = solve(input)
end_time = Time.now

puts "Solution: #{solution}"
puts "Completed in #{end_time - start_time} seconds"
