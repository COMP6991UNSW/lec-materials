#include <stdio.h>
#include <stdlib.h>

int main(void) {
	FILE *file = fopen("foo.txt", "r");
	
	char line[1024];
	fgets(line, sizeof line, file);
	
	printf("File has: %s\n", line);

	// fclose(file);
}

void print_first_line_of_file(FILE *file) {
	// ...

	fclose(file);
}
