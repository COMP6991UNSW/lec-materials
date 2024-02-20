import java.io.*;
import java.util.ArrayList;
import java.util.List;

public class Main {
    public void main() throws IOException {
        var firstLines = new ArrayList<String>();

        var fileNames = List.of("hello.txt", "world.txt", "foo.bar");
        for (var name : fileNames) {
            var firstLine = firstLineOfFile(name);

            firstLines.add(firstLine);
        }

        System.out.println(firstLines);
    }

    public String firstLineOfFile(String fileName) throws IOException {
        var file = new File(fileName);
        var reader = new BufferedReader(new FileReader(file));
        var line = reader.readLine();

        return line;
    }



    public static void main(String[] args) throws IOException {
        new Main().main();
    }
}