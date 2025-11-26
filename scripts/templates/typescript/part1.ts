import * as fs from 'fs';
import * as path from 'path';

function readInput(filePath: string): string[] {
  return fs.readFileSync(filePath, 'utf8').split('\n').filter(line => line.length > 0);
}

function solve(input: string[]): string {
  // Your solution for part 1 goes here
  return "solution for part 1";
}

const startTime = new Date().getTime();
const inputFile = process.argv.length > 2 ? process.argv[2] : 'input.txt';
const input = readInput(inputFile);
const solution = solve(input);
const endTime = new Date().getTime();

console.log(`Solution: ${solution}`);
console.log(`Completed in ${(endTime - startTime) / 1000} seconds`);
