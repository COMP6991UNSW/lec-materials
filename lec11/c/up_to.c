// Examples from: https://www.cs.yale.edu/homes/aspnes/pinewiki/C(2f)Macros.html

#include <stdio.h>
#include <stdlib.h>

#define UP_TO(type, i, n) for (type (i) = 0; (i) < (n); (i)++)

int main(void) {
    UP_TO(int, foo, 10) {
        printf("%d\n", foo);
    }
}
