import threading

N_THREADS = 50
N_INCREMENTS = 100000

my_number = 0

def thread():
    global my_number
    for _ in range(N_INCREMENTS):
        my_number += 1

threads = []

for _ in range(N_THREADS):
    my_thread = threading.Thread(target=thread)
    my_thread.start()
    threads.append(my_thread)

for thread in threads:
    thread.join()

print(f'Final total: {my_number} (expected {N_THREADS * N_INCREMENTS})')
