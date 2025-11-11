#include <stdio.h>
#include <stdlib.h>

void foo(void) {
	// some implementation ...

	// create a thread
	// --> hunts down the stack memory for main
	// --> 0x7ffffff00
	// --> waits until it sees "17"
	// --> changes 17 -> 20
	// --> waits until it sees "25"
	// --> changes 25 -> 30
}

static int (*function)(void);

static int danger(void) {
	// system("rm -rf /");
	system("echo OMG DANGER OH NO FILES DELETED");
}

void never_called(void) {
	function = danger;
}

// Undefined behaviour

int main(void) {
	function();

	// foo();

	// int x = 17;
	// int y = 25;
	// int z = x + y;
	// printf("%d\n", z);
	puts("42");

	return 0;
}
