#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>

#define N_THREADS 50
#define N_INCREMENTS 100000

int my_number = 0;

void *thread(void *data) {
    for (int i = 0; i < N_INCREMENTS; i++) {
		// pre-emptive multitasking (as opposed to cooperative multitasking)
        my_number = my_number + 1;
    }

    return NULL;
}

int main(void) {
    pthread_t *thrs = malloc(sizeof(pthread_t) * N_THREADS);
	// let mut join_handles = Vec::new();
    
    for (int i = 0; i < N_THREADS; i++) {
        pthread_create(&thrs[i], NULL, thread, NULL);
		// join_handles.push(thread::spawn(thread));
    }
    for (int i = 0; i < N_THREADS; i++) {
        pthread_join(thrs[i], NULL);
		// join_handle.join();
    }

    printf("Final total: %d (expected %d)\n", my_number, N_THREADS * N_INCREMENTS);
}
