#include <stdio.h>

struct MyFile {
	FILE *file;
	bool is_closed;
}

// I will clean up file, don't use it after calling me
//
// I will not clean up file, make sure to fclose after me!
void takesInAFile(FILE *file) {
	// ...
	//
	//
	
	// should this fn fclose(file)?
}

int main(void) {
    FILE *file = fopen("hello.txt", "r");
    char line[1024];
    fgets(line, sizeof line, file);

    printf("%s", line);

    takesInAFile(file);

    fclose(file);
}
