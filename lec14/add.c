#include <stdio.h>

int main(void) {
	double sum = 0.0;
	for (double i = 0; i < 10000000000; i++) {
		sum += i;
	}
	printf("%lf\n", sum);
}
