package main

import (
    "fmt"
    "sync"
)

const NThreads = 50
const NIncrements = 100000

func main() {
    var mutex sync.Mutex

    var wg sync.WaitGroup
    wg.Add(NThreads)

    my_number := 0

    for i := 0; i < NThreads; i++ {
        go func() {
            for n := 0; n < NIncrements; n++ {
                mutex.Lock()
                my_number++
                mutex.Unlock()
            }
            wg.Done()
        }()
    }

    wg.Wait()

    fmt.Printf("Final total: %d (expected %d)\n", my_number, NThreads * NIncrements)
}

