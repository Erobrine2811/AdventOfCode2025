package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"time"
)

func readInput(filePath string) []string {
	file, err := os.Open(filePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return lines
}

func main() {
	startTime := time.Now()

	if len(os.Args) < 3 {
		log.Fatalf("Usage: go run . <part> <input_file>")
	}

	part := os.Args[1]
	inputFile := os.Args[2]

	input := readInput(inputFile)
	var solution string

	if part == "part1" {
		solution = solvePart1(input)
	} else if part == "part2" {
		solution = solvePart2(input)
	} else {
		log.Fatalf("Invalid part: %s", part)
	}

	duration := time.Since(startTime)

	fmt.Printf("Solution: %s\n", solution)
	fmt.Printf("Completed in %.4f seconds\n", duration.Seconds())
}