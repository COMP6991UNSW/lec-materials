#include <stdio.h>

int main(void) {
    FILE *file = fopen("hello.txt", "r");
    if (file == NULL) {
        // ...
    }
    char line[1024];
    fgets(line, sizeof line, NULL);

    printf("The line is: %s\n", line);
}