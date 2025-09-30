int read_line(char *str, int size) {
    // do some stuff to read a line
    if (error) {
        return 1;
    }
}

int *pointer_return() {
    int x = 3;
    return &x;
}

int main(void) {
    int x = 3;
    if (x = 2) {
        printf("Hello world!\n");
    }

    int a = 2;
    int b = 3;
    a = (b = 7);


    char *my_string = "...";
    int length;
    while (length = read_line(my_string, 3)) {

    }
}