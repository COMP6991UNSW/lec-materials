pub enum CarBrand {
	Toyota,
	Subaru,
	Nissan,
}

pub trait Car {
	fn brand(&self) -> CarBrand;
	fn horsepower(&self) -> u32;
}

car!(Corolla,  Toyota, 100);
car!(Cressida, Toyota, 160);
car!(Chaser,   Toyota, 220);

car!(Liberty,  Subaru, 100);
car!(Impreza,  Subaru, 130);
car!(Wrx,      Subaru, 200);

car!(Pulsar,   Nissan, 90);
car!(Silvia,   Nissan, 200);
car!(Skyline,  Nissan, 220);

#[macro_export]
macro_rules! car {
	($name:ident, $brand:expr, $horsepower:literal) => {
		pub struct $name;
		impl Car for $name {
			fn brand(&self) -> CarBrand {
				use CarBrand::*;
				$brand
			}
			fn horsepower(&self) -> u32 {
				$horsepower
			}
		}
	}
}
