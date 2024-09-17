#include <stdio.h>


func openFile() (file, err) { ... }



int main(void) {
	file, err = openFile();
	if err != nil {
		// file: <undefined>
		// ...
	}
	file
}
