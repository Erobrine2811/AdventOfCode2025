startTime <- Sys.time()

read_input <- function(filePath) {
  lines <- readLines(filePath, warn = FALSE)
  return(lines[nchar(lines) > 0])
}

solve <- function(input) {
  # Your solution for part 1 goes here
  return("solution for part 1")
}

args <- commandArgs(trailingOnly = TRUE)
inputFile <- if (length(args) > 0) args[1] else "input.txt"
input <- read_input(inputFile)
solution <- solve(input)

endTime <- Sys.time()
duration <- as.numeric(endTime - startTime, units = "secs")

cat("Solution:", solution, "\n")
cat(sprintf("Completed in %.4f seconds\n", duration))