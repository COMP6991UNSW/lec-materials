use cars::*;

fn favourite_car(brand: CarBrand) -> impl Car {
	use CarBrand::*;

	match brand {
		Toyota => Cressida,
		Subaru => Liberty,
		Nissan => Skyline,		
	}
}

fn main() {
	// TODO
}
