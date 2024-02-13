// build with: javac Option.java
// run with: java Option

import java.util.Optional;

class Option {
	static Optional<String> create(boolean b) {
		if (b) {
			return Optional.of("Hello there!");
		} else {
			return Optional.empty();
		}
	}

	public static void main(String[] args) {
		Optional<String> create_true = create(true);
		if (create_true.isPresent()) {
			System.out.println("create(true) returned " + create_true.get());
		}

		System.out.println(
				"create(false) returned "
				+ create(false).orElse("<empty>")
		);
	}
}
