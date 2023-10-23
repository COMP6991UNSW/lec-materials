// Examples from: https://www.cs.yale.edu/homes/aspnes/pinewiki/C(2f)Macros.html

#include <stdio.h>
#include <stdlib.h>

// This macro should only be used with types
// that are comparable (i.e. a < b)
#define MAX(a, b) (a) > (b) ? (a) : (b)
#define MUL2(a) (a) * 2
int imax(int a, int b) { return a > b ? a : b; }
double fmax(double a, double b) { return a > b ? a : b; }

int dbg(int x) {
	printf("DEBUG: %d\n", x);
	return x;
}

int main(void) {
	int x = MAX(10, MUL2(9 + 10));
	printf("The max is %d\n", x);
}
