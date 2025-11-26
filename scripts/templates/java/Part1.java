import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.stream.Collectors;

public class Part1 {
    public static List<String> readInput(String filePath) throws IOException {
        return Files.lines(Paths.get(filePath)).filter(s -> !s.isEmpty()).collect(Collectors.toList());
    }

    public static String solve(List<String> input) {
        // Your solution for part 1 goes here
        return "solution for part 1";
    }

    public static void main(String[] args) {
        long startTime = System.nanoTime();
        
        String inputFile = "input.txt";
        if (args.length > 0) {
            inputFile = args[0];
        }

        try {
            List<String> input = readInput(inputFile);
            String solution = solve(input);
            long endTime = System.nanoTime();
            double duration = (endTime - startTime) / 1_000_000_000.0;
            
            System.out.println("Solution: " + solution);
            System.out.printf("Completed in %.4f seconds\n", duration);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
