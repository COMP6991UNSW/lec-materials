#include <stdio.h>

void foo(void) {
	// look up main's memory
	// search for 17, changes it to 100
	// search for 25, changes it to 200
}

int main(void) {
	foo();

	int x = 17;
	int y = 25;

	printf("%d\n", x + y);
}
