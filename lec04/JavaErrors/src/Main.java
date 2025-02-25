import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;

public class Main {
    void main() {
        var fileName = "foo.txt";
        var line = readLineFromFile(fileName);

        System.out.println(line);
    }

    String readLineFromFile(String fileName) {
        var file = new File(fileName);
        var reader = new BufferedReader(new FileReader(file));
        return reader.readLine();
    }










    public static void main(String[] args) {
        new Main().main();
    }
}