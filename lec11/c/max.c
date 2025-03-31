#include <stdio.h>

#define ADD5(x) ((x) + 5)
#define MUL7(x) ((x) * 7)

#define MAX(a, b) (((a) > (b)) ? (a) : (b))

int add5(int x) { return x + 5; }

int max(int a, int b) {
	return a > b ? a : b;
}

// T dbg(T value) {
// 	printf("{value:?}");
// 	return value;
// }

int dbg(int value) {
	printf("DBG: %d\n", value);
	return value;
}

int main(void) {
	int x = MAX(dbg(5 + 5), dbg(21 * 2));
	printf("%d\n", x);
}
