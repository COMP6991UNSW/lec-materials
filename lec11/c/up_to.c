// Examples from: https://www.cs.yale.edu/homes/aspnes/pinewiki/C(2f)Macros.html

#include <stdio.h>
#include <stdlib.h>

// TODO
#define UP_TO(type, i, n) for (type i = 0; i < n; i++)

int main(void) {
    UP_TO(int, foo, 10) {
        printf("%d\n", foo);
    }
    
    // should make:
    // for (int foo = 0; foo < 10; foo++) {
    //     printf("%d\n", foo);
    // }
}
