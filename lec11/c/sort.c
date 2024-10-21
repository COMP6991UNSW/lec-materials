// Examples from: https://www.cs.yale.edu/homes/aspnes/pinewiki/C(2f)Macros.html

#include <stdio.h>
#include <stdlib.h>

#define DeclareSort(prefix, type) \
static int \
_DeclareSort_ ## prefix ## _Compare(const void *a, const void *b) \
{ \
    const type *aa; const type *bb; \
    aa = a; bb = b; \
    if(*aa < *bb) return -1; \
    else if(*bb < *aa) return 1; \
    else return 0; \
} \
\
void \
prefix ## _sort(type *a, int n)\
{ \
    qsort(a, n, sizeof(type), _DeclareSort_ ## prefix ## _Compare); \
}

DeclareSort(int, int)
DeclareSort(float, float)
DeclareSort(double, double)
DeclareSort(char, char)

int main(void) {
    int array[] = { 6, 1, 3, 9, 10, 5, 2, 4, 8, 7 };
    int_sort(array, 10);

    for (int i = 0; i < 10; i++) {
        printf("%d\n", array[i]);
    }
}
