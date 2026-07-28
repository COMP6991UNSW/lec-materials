#include <stdlib.h>
#include <stdio.h>

static void (*function)(void);
// static FUNCTION: fn();

void danger(void) {
	system("echo DANGER DANGER");
}

void do_not_run(void) {
	function = danger;
}

int main(void) {
	printf("hello\n");
	function();
}
