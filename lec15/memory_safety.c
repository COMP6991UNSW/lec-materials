#include <stdio.h>

int return_value(void) {
	int x = 25;
	int y = 17;
	int z = x + y;

	return z;
}

int main(void) {
	int value = return_value();
	printf("%d\n", value);
}
