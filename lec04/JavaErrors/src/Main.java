import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class Main {
    public void main() throws IOException {
        var list1 = new ArrayList<String>();

        var files = List.of("foo.txt", "bar.txt", "baz.txt");
        for (String file : files) {
            var line = readLineFromFile(file);
            list1.add(line);
        }

        var list2 = files.stream()
                .map(name -> {
                    try {
                        return readLineFromFile(name);
                    } catch (IOException e) {
                        throw new RuntimeException(e);
                    }
                })
                .toList();
    }

    public String readLineFromFile(String fileName) throws IOException {
        var file = new File(fileName);
        var reader = new BufferedReader(new FileReader(file));
        return reader.readLine();
    }




    public static void main(String[] args) throws IOException {
        new Main().main();
    }
}