import java.util.ArrayList;
import java.util.List;

public class Foo {
	public static void main(String[] args) {
		var list = new ArrayList<>(
			List.of(1, 2, 3)
		);

		// move
		// shared borrow
		for (var elem : list) {
			System.out.println(elem);
			// another exclusive borrow
			list.add(42);
		}
	}
}
