#include <stdio.h>

void foo(void);

int main(void) {
	int x = 17;
	int y = 25;

	foo();

	printf("%d\n", x + y);
}

void foo(void) {
	// look up main's memory
	// search for 17, changes it to 100
	// search for 25, changes it to 200
	
	int x;
	int *ptr = &x;
	for (int *curr = ptr; curr >= ptr - 100; ptr--) {
		if (*curr == 17) {
			*curr = 100;
		}
		if ..
	}

	// Implementation left as an
	// exercise to the reader :^)
}
