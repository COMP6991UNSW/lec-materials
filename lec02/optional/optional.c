// build with clang optional.c -o optional

#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

char *create(bool b) {
	if (b) {
		return "Hello there";
	} else {
		return NULL;
	}
}

int main(void) {
	// method 1
	char *create_true = create(true);
	if (create_true != NULL) {
		printf("create(true) returned %s\n", create_true);
	}

	// method 2
	char *create_false = create(false);
	printf(
		"create(false) returned %s\n",
		create_false != NULL ? create_false : "<empty>"
	);
}
