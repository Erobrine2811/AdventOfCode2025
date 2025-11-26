<?php

function readInput($filePath) {
    return array_filter(explode("\n", file_get_contents($filePath)));
}

function solve($input) {
    // Your solution for part 1 goes here
    return "solution for part 1";
}

$startTime = microtime(true);
$inputFile = $argc > 1 ? $argv[1] : 'input.txt';
$input = readInput($inputFile);
$solution = solve($input);
$endTime = microtime(true);

$duration = $endTime - $startTime;

echo "Solution: $solution\n";
echo "Completed in $duration seconds\n";

