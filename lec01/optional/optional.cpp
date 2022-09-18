// Build with: clang++ -std=c++17 optional.cpp -o optional_cpp

#include <optional>
#include <iostream>

std::optional<std::string> create(bool b) {
	if (b) {
		return std::optional<std::string>{"Hello there!"};
	} else {
		return std::nullopt;
	}
}

int main()
{
	// method one
	auto create_true = create(true);
	if (create_true) {
		std::cout << "create(true) returned "
			      << *create_true
			      << '\n';
	}

	// method two
	std::cout << "create(false) returned "
		      << create(false).value_or("<empty>")
		      << '\n';
}

