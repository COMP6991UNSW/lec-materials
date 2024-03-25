#include <stdio.h>
#include <stdlib.h>

#define X 42
#define ADD1(X) ((X) + 1)
#define MUL2(X) ((X) * 2)
#define MAX(A, B) (((A) > (B)) ? (A) : (B))

int max(int a, int b) {
	return a > b ? a : b;
}

int dbg(int x) {
	printf("DEBUG: %d\n", x);
	return x;
}

int main(void) {
	int foo = X;
	int bar = ADD1(dbg(5)) * 2;
	printf("%d\n", bar);

	int x = max(dbg(10), 42);
	double y = max(3.14, 1.23);
	printf("%d %lf\n", x, y);

	int a = MAX(10, dbg(42));
	double b = MAX(3.14, 1.23);
	printf("%d %lf\n", a, b);

	char *m = MAX("foo", "bar");
	printf("%s\n", m);
}
