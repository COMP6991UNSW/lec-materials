my_function() -> (ValuePtr, i32) {
	// ...
}

main() {
	let (value, ok) = my_function();
	if ok != 0 {
		// ...
		// ...
		return;
	}

	value
}
