// Examples from: https://www.cs.yale.edu/homes/aspnes/pinewiki/C(2f)Macros.html

#include <stdio.h>
#include <stdlib.h>

#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define MULTIPLY_BY_7(x) (x) * 7
#define ADD_5(x) ((x) + 5)

int print_and_return(int x) {
    printf("The int is %d\n", x);
    return x;
}

int main(void) {
    int   imax = MAX(print_and_return(10), print_and_return(42));
    float fmax = MAX(3.14, 2.78);

    printf("%d, %f\n", imax, fmax);

    int my_number = MULTIPLY_BY_7(2 + 2);
    printf("%d\n", my_number);

    my_number = ADD_5(2) * 4;
    printf("%d\n", my_number);
}
