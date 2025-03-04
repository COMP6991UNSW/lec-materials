#include <stdio.h>
#include <stdlib.h>

struct node {
	int data;
	struct node *next;
};

struct node *create_list(int first_data) {
	struct node head;
	head.data = first_data;
	head.next = NULL;
	return &head;
}

int main(void) {
	struct node *list = create_list(42);
	list->next = create_list(43);
	list->next->next = create_list(44);
}
