#include <stdio.h>

#define ADD5(var) ((var) + 5)
#define MUL2(var) ((var) * 2)
#define MAX(a, b) (((a) > (b)) ? (a) : (b))

// int add5(int x) { return x + 5; }
// int max(int a, int b) {
// 	return a > b ? a : b;
// }

int main(void) {
	int x = MAX(2 * ADD5(3), 15);
	
	int a = MUL2(5 + 3);
	printf("%d\n", a);

	float y = MAX(ADD5(3.5), 5.5);
	printf("%d %lf\n", x, y);
}
