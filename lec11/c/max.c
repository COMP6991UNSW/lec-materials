#include <stdio.h>

#define ADD5(x) ((x) + 5)
#define MUL7(x) ((x) * 7)
#define MAX(a, b) (((a) > (b)) ? (a) : (b))

// int add5(int x) { return x + 5; }

// int max(int a, int b) {
// 	// if (a > b) { return a; } else { return b; }
// 	return a > b ? a : b;
// }

int main(void) {
	int value = MUL7(2 + 2);

	int max = MAX(3 * ADD5(2), 7);

	printf("%d %d\n", max, value);
}
