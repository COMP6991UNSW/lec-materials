#include <stdio.h>

void do_stuff(void) {
	// spawn a new thread
	// wait for that thread to kick off
	// look at main's memory (in a loop)
	// look for the value "17" anywhere
	// change that to "25"
}

int main(void) {
	char *first = strtok("abc:def:ghi", ":"); // "abc"
	char *second = strtok(NULL, ":"); // "def"
	char *third = strtok(NULL, ":"); // "ghi"
	char *fourth = strtok(NULL, ":"); // NULL

	do_stuff();

	int x = 25;
	int y = 25;
	puts("42");
}
