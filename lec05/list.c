#include <stdio.h>
#include <stdlib.h>

struct node {
	int data;
	struct node *next;
};

/// This function reutrns a pointer that
/// you must free when finished with it
struct node *create_list(int data) {
	struct node new;
	new.data = data;
	new.next = NULL;
	return &new;

	struct node *new = malloc(sizeof(*new));
	new->data = data;
	new->next = NULL;
	return new;
}
