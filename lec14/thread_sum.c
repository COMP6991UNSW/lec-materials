#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

struct job {
    long start, finish;
    double sum;
};

void *run_thread(void *argument) {
    struct job *j = argument;
    long start = j->start;
    long finish = j->finish;
    double sum = 0;

    for (long i = start; i < finish; i++) {
        sum += i;
    }

    j->sum = sum;

    printf("Thread summing integers %10lu to %11lu finished sum is %20.0f\n",
           start, finish, sum);
    return NULL;
}

int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "Usage: %s <n-threads> <n-integers-to-sum>\n", argv[0]);
        return 1;
    }

    int n_threads = strtol(argv[1], NULL, 0);
    assert(0 < n_threads && n_threads < 1000);
    long integers_to_sum = strtol(argv[2], NULL, 0);
    assert(0 < integers_to_sum);

    long integers_per_thread = (integers_to_sum - 1) / n_threads + 1;

    printf("Creating %d threads to sum the first %lu integers\n"
           "Each thread will sum %lu integers\n",
           n_threads, integers_to_sum, integers_per_thread);

    pthread_t thread_id[n_threads];
    struct job jobs[n_threads];

    for (int i = 0; i < n_threads; i++) {
        jobs[i].start = i * integers_per_thread;
        jobs[i].finish = jobs[i].start + integers_per_thread;

        if (jobs[i].finish > integers_to_sum) {
            jobs[i].finish = integers_to_sum;
        }

        // create a thread which will sum integers_per_thread integers
        pthread_create(&thread_id[i], NULL, run_thread, &jobs[i]);
    }

    // Wait for threads to finish, then add results for an overall sum.
    double overall_sum = 0;
    for (int i = 0; i < n_threads; i++) {
        pthread_join(thread_id[i], NULL);
        overall_sum += jobs[i].sum;
    }

    printf("\nCombined sum of integers 0 to %lu is %.0f\n", integers_to_sum,
           overall_sum);
    return 0;
}
