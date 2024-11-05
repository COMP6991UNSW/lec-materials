#include <stdio.h>

int *foo(void) {
	int x = 42;
	return &x;
}

int main(void) {
	int *ptr = foo();

	printf("%d\n", *ptr);
}
