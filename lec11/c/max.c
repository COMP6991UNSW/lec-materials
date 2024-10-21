#include <stdio.h>
#include <stdlib.h>

#define PI 3.14
#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define ADD5(x) ((x) + 5)
#define MUL7(x) ((x) * 7)

int dbg(int x) {
	printf("DBG: %d\n", x);
	return x;
}

int main(void) {
	int a = (ADD5(42));
	int b = MUL7(3);
	int c = MUL7(ADD5(1));
	int d = MUL7(2 + 2);
	printf("%d %d %d %d\n", a, b, c, d);

	int x = MAX(dbg(10), dbg(42));
	double y = MAX(1.23, PI);
	int z = MAX(1 + 2, 3 + 4);
	printf("%d %lf %d\n", x, y, z);
}
