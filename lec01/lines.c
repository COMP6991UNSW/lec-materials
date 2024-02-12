// Gives us `struct image`, `struct pixel`, `save_image`
#include "bmp.h"

#define IMAGE_SIZE_PX 256

struct pixel create_pixel(int r, int g, int b);

int main(void) {
	struct image my_image;
	my_image.width  = IMAGE_SIZE_PX;
	my_image.height = IMAGE_SIZE_PX;

	struct pixel red = create_pixel(255, 0, 0);
	struct pixel green = create_pixel(0, 255, 0);
	struct pixel blue = create_pixel(0, 0, 255);
	struct pixel black = create_pixel(0, 0, 0);

	for (int x = 0; x < IMAGE_SIZE_PX; x++) {
		for (int y = 0; y < IMAGE_SIZE_PX; y++) {
			if ((x + y) % 15 == 0) {
				set_pixel(image, x, y, red);
			} else if ((x + y) % 15 == 5) {
				set_pixel(image, x, y, green);
			} else if ((x + y) % 15 == 10) {
				set_pixel(image, x, y, blue);
			} else {
				set_pixel(image, x, y, black);
			}
		}
	}

	int result = save_image(image, "my_image.bmp");
	if (result != 0) {
		fputs(stderr, "Failed to save image!\n");
		return 1;
	}

	return 0;
}

struct pixel create_pixel(int r, int g, int b) {
	struct pixel pixel;

	pixel.r = r;
	pixel.g = g;
	pixel.b = b;

	return pixel;
}

