#include <stdio.h>
#include <stdlib.h>

void print_int(int *ptr);
int *give_me_int(void);

int main(void) {
	int *ptr = give_me_int();

	// ...
	print_int(ptr);

	free(ptr);
}

void print_int(int *ptr) {
	printf("%d\n", *ptr);
	free(ptr);
}

int *give_me_int(void) {
	int *ptr = malloc(sizeof(int));
	*ptr = 42;
	return ptr;
}
