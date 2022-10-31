import threading
N_THREADS = 50
N_INCREMENTS = 100000

my_number = 0

def thread():
    for _ in range(N_INCREMENTS):
        global my_number
        my_number += 1

for _ in range(N_THREADS):
    threading.Thread(target=thread).start()

print(my_number)
