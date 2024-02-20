#include <stdio.h>

void do_something_with_file(FILE *file) {
	// ...
}

int main(void) {
    FILE *file = fopen("hello.txt", "r");
	if (file == NULL) {
		
	}
    char line[1024];
    if (fgets(line, sizeof line, file) == NULL) {
		
	}

    printf("%s", line);

	do_something_with_file(file);
}
