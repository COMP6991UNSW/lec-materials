// Examples from: https://www.cs.yale.edu/homes/aspnes/pinewiki/C(2f)Macros.html

#include <stdio.h>
#include <stdlib.h>

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MULTIPLY_BY_7(x) ((x) * 7)
#define ADD_5(x) ((x) + 5)

int max(int a, int b) { return a > b ? a : b; }
int multiply_by_7(int x) { return x * 7; }
int add_5(int x) { return x + 5; }

int main(void) {
	int x = MULTIPLY_BY_7(2 + 2);
	printf("%d\n", x);

	int y = ADD_5(2) * 4;
	printf("%d\n", y);

	// MULTIPLY_BY_7(int *foo = );
}
