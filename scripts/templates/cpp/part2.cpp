#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <chrono>

std::vector<std::string> read_input(const std::string& filename) {
    std::vector<std::string> lines;
    std::ifstream file(filename);
    std::string line;
    while (std::getline(file, line)) {
        if (!line.empty()) {
            lines.push_back(line);
        }
    }
    return lines;
}

std::string solve(const std::vector<std::string>& input) {
    // Your solution for part 2 goes here
    return "solution for part 2";
}

int main(int argc, char* argv[]) {
    auto start_time = std::chrono::high_resolution_clock::now();

    std::string input_file = "input.txt";
    if (argc > 1) {
        input_file = argv[1];
    }

    auto input = read_input(input_file);
    std::string solution = solve(input);

    auto end_time = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> duration = end_time - start_time;

    std::cout << "Solution: " << solution << std::endl;
    std::cout << "Completed in " << duration.count() << " seconds" << std::endl;

    return 0;
}
