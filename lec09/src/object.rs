pub(crate) fn main() {
    let haku = Dog {
        name: String::from("Haku"),
        breed: DogBreed::GoldenRetriever
    };

    let betty = Dog {
        name: String::from("Betty"),
        breed: DogBreed::Pug,
    };

    let wilson = Cat {
        name: String::from("Wilson"),
        is_happy: true,
    };

    let andrew = Frog {
        name: String::from("Andrew")
    };

    let haku_borrow  = Box::new(haku);
    let betty_borrow  = Box::new(betty);
    let wilson_borrow = Box::new(wilson);
    let andrew_borrow = Box::new(andrew);

    let my_animals: Vec<Box<dyn Animal>> = vec![
        haku_borrow,
        betty_borrow,
        wilson_borrow,
        andrew_borrow
    ];

    println!("{}", collate_summaries(&my_animals));
}

fn collate_summaries(animals: &[Box<dyn Animal>]) -> String {
    let mut summary = String::new();

    for animal in animals {
        summary.push_str(&summarize_animal(&**animal));
        summary.push('\n');
    }

    summary
}

fn summarize_animal(animal: &dyn Animal) -> String {
    let name = animal.name();

    format!(
        "{name} says {} \
         {name} has {} legs \
         {name} has {}fur",
        animal.speak(),
        animal.n_legs(),
        if animal.has_fur() { "" } else { "no " }
    )
}


trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn n_legs(&self) -> u32;
    fn has_fur(&self) -> bool;
}

impl<A> Animal for &A
where
    A: Animal
{
    fn name(&self) -> &str {
        A::name(self)
    }

    fn speak(&self) -> String {
        A::speak(self)
    }

    fn n_legs(&self) -> u32 {
        A::n_legs(self)
    }

    fn has_fur(&self) -> bool {
        A::has_fur(self)
    }
}

impl Animal for &dyn Animal {
    fn name(&self) -> &str {
        (self as &dyn Animal).name()
    }

    fn speak(&self) -> String {
        (self as &dyn Animal).speak()
    }

    fn n_legs(&self) -> u32 {
        (self as &dyn Animal).n_legs()
    }

    fn has_fur(&self) -> bool {
        (self as &dyn Animal).has_fur()
    }
}


struct Dog {
    name: String,
    breed: DogBreed,
}

enum DogBreed {
    GoldenRetriever,
    Labrador,
    Pug,
    // ...
}

struct Cat {
    name: String,
    is_happy: bool,
}

struct Frog {
    name: String,
}




impl Animal for Dog {
    fn speak(&self) -> String {
        match self.breed {
            DogBreed::GoldenRetriever
                | DogBreed::Labrador => String::from("WOOF!"),
            DogBreed::Pug => String::from("arf!"),
        }
    }

    fn n_legs(&self) -> u32 {
        4
    }

    fn has_fur(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        if self.is_happy {
            String::from("meow :)")
        } else {
            String::from("hISS")
        }
    }

    fn n_legs(&self) -> u32 {
        4
    }

    fn has_fur(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Frog {
    fn speak(&self) -> String {
        String::from("ribbot")
    }

    fn n_legs(&self) -> u32 {
        4
    }

    fn has_fur(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        &self.name
    }
}








