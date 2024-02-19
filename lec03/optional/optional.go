// run with `go run optional.go`

package main

import "fmt"

func strPtr(s string) *string {
	return &s
}

func create(b bool) *string {
	if (b) {
		return strPtr("Hello there")
	} else {
		return nil
	}
}

func main() {
	// method 1
	create_true := create(true)
	if (create_true != nil) {
		fmt.Printf("create(true) returned %s\n", *create_true)
	}

	// method 2
	create_false := create(false)
	var string_to_print string

	if (create_false != nil) {
		string_to_print = *create_false
	} else {
		string_to_print = "<empty>"
	}

	fmt.Printf("create(false) returned %s\n", string_to_print)
}
