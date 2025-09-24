import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.List;

public class Main {
    public static void main(String[] args) throws IOException {
        var files = List.of("foo.txt", "bar.txt", "baz.txt");

        List<String> firstLines = files.stream()
                .map(file -> printFirstLineOfFile(file))
                .toList();

        for (var file : files) {
            System.out.println(printFirstLineOfFile(file));
        }
    }

    static String printFirstLineOfFile(String fileName) throws IOException {
        var file = new File(fileName);
        var reader = new BufferedReader(new FileReader(file));

        String line = reader.readLine();
        return line;
    }
}