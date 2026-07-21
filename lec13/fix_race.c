#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>

#define N_THREADS 50
#define N_INCREMENTS 100000

// Make sure to lock `mutex` before accessing or mutating this value!!!!
// Please please please pretty please!!!!
int my_number = 0;
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

void *thread(void *data) {
    for (int i = 0; i < N_INCREMENTS; i++) {
		pthread_mutex_lock(&mutex);
        my_number += 1;
		pthread_mutex_unlock(&mutex);
    }

    return NULL;
}

int main(void) {
    pthread_t *thrs = malloc(sizeof(pthread_t) * N_THREADS);
    
    for (int i = 0; i < N_THREADS; i++) {
        pthread_create(&thrs[i], NULL, thread, NULL);
    }
    for (int i = 0; i < N_THREADS; i++) {
		pthread_mutex_lock(&mutex);
		int the_value = my_number;
		pthread_mutex_unlock(&mutex);
		printf("Final total: %d (expected %d)\n", the_value, N_THREADS * N_INCREMENTS);
        pthread_join(thrs[i], NULL);
    }


    printf("Final total: %d (expected %d)\n", my_number, N_THREADS * N_INCREMENTS);
    
    free(thrs);
}
