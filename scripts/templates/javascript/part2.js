const fs = require('fs');

function readInput(filePath) {
  return fs.readFileSync(filePath, 'utf8').split('\n').filter(line => line.length > 0);
}

function solve(input) {
  // Your solution for part 2 goes here
  return "solution for part 2";
}

const startTime = Date.now();
const inputFile = process.argv.length > 2 ? process.argv[2] : 'input.txt';
const input = readInput(inputFile);
const solution = solve(input);
const endTime = Date.now();

console.log(`Solution: ${solution}`);
console.log(`Completed in ${(endTime - startTime) / 1000} seconds`);
