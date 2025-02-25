#include <stdio.h>

void do_something_with_file(FILE *file);

int main(void) {
	FILE *file = fopen("foo.txt", "r");

	char line[1024];
	fgets(line, sizeof line, file);
	printf("%s", line);

	do_something_with_file(file);
}

void do_something_with_file(FILE *file) {
	// TODO
}
