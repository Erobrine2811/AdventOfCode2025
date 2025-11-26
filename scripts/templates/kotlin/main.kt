import java.io.File
import java.math.BigInteger
import kotlin.system.measureTimeMillis

fun readInput(fileName: String) = File(fileName).readLines()

fun main(args: Array<String>) {
    val part = args.getOrNull(0) ?: "part1"
    val inputFile = args.getOrNull(1) ?: "input.txt"

    val input = readInput(inputFile)
    
    val solution: String
    val time = measureTimeMillis {
        solution = when (part) {
            "part1" -> solvePart1(input)
            "part2" -> solvePart2(input)
            else -> "Invalid part"
        }
    }

    println("Solution: $solution")
    println("Completed in ${time / 1000.0} seconds")
}
