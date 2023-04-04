/*

SOURCED FROM COMP1521

Simple example of dividing a task between `n' threads.

Compile like:

    $ gcc -O3 -pthread thread_sum.c -o thread_sum

One thread takes 10 seconds:

    $ time ./thread_sum 1 10000000000
    Creating 1 threads to sum the first 10000000000 integers
    Each thread will sum 10000000000 integers
    Thread summing integers          0 to 10000000000 finished sum is 49999999990067863552

    Combined sum of integers 0 to 10000000000 is 49999999990067863552

    real    0m11.924s
    user    0m11.919s
    sys 0m0.004s
    $

Four threads runs 4x as fast on a machine with 4 cores:

    $ time ./thread_sum 4 10000000000
    Creating 4 threads to sum the first 10000000000 integers
    Each thread will sum 2500000000 integers
    Thread summing integers 2500000000 to  5000000000 finished sum is  9374999997502005248
    Thread summing integers 7500000000 to 10000000000 finished sum is 21874999997502087168
    Thread summing integers 5000000000 to  7500000000 finished sum is 15624999997500696576
    Thread summing integers          0 to  2500000000 finished sum is  3124999997567081472

    Combined sum of integers 0 to 10000000000 is 49999999990071869440

    real    0m3.154s
    user    0m12.563s
    sys 0m0.004s
    $

Note the result is inexact, because we use values can't be exactly
represented as double and exact value printed depends on how many
threads we use - because we break up the computation differently
depending on the number of threads.

*/

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
