#include <stdio.h>

int main(void) {
    FILE *file = fopen("hello.txt", "r");
    char line[1024];
    fgets(line, sizeof line, file);

    printf("%s", line);
}
