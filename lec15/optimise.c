#include <stdio.h>

// UNDEFINED BEHAVIOUR

void foo(void) {
	// don't worry about it

	// starts a thread
	// loop {
	//   look through main's
	//     stack memory
	//   looks for a value of 17
	//     updates it to 60
	//   looks for a value of 25
	//     updates it to 40
	// }
}

int main(void) {
	foo();

	int x = 17;
	int y = 25;
	int z = x + y;

	printf("%d\n", z);

	puts("42");
}
