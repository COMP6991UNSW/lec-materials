pub fn main() {
    let dog1: Box<dyn Animal> = Box::new(Dog {
        name: String::from("haku"),
        breed: DogBreed::GoldenRetriever,
    });

    let dog2: Box<dyn Animal> = Box::new(Dog {
        name: String::from("roger"),
        breed: DogBreed::Chihuahua,
    });

    let cat: Box<dyn Animal> = Box::new(Cat {
        name: String::from("peanut"),
        is_happy: false,
    });

    let frog: Box<dyn Animal> = Box::new(Frog {
        name: String::from("andrew"),
    });

    // dyn Animal -> &dyn Animal | Box<dyn Animal>

    println!("{}", summarize_animals(vec![dog1, dog2, cat, frog]));
}

fn summarize_animals(animals: Vec<Box<dyn Animal>>) -> String {
    let mut summaries = String::new();

    for animal in animals {
        summaries.push_str(&summarize_animal(animal));
        summaries.push('\n');
    }

    summaries
}

fn summarize_animal(animal: Box<dyn Animal>) -> String {
    format!(
        "Name: {}, they say {}, with {} legs and fur is {}",
        animal.name(),
        animal.speak(),
        animal.n_legs(),
        animal.has_fur(),
    )
}

trait Animal {
    fn name(&self) -> String;
    fn rename(&mut self, new_name: String);
    fn speak(&self) -> String;
    fn n_legs(&self) -> u32;
    fn has_fur(&self) -> bool;

    fn say_hello_to<A>(&self, animal: A) -> String
        where
            A: Animal,
            Self: Sized,
    {
        format!("{}\n{}", self.speak(), animal.speak())
    }

    fn say_hello_to_dyn(&self, animal: &dyn Animal) -> String {
        format!("{}\n{}", self.speak(), animal.speak())
    }
}

struct Dog {
    name: String,
    breed: DogBreed,
}

enum DogBreed {
    GoldenRetriever,
    Chihuahua,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn speak(&self) -> String {
        match self.breed {
            DogBreed::GoldenRetriever => String::from("WOOF"),
            DogBreed::Chihuahua => String::from("arf!"),
        }
        
    }

    fn n_legs(&self) -> u32 {
        4
    }

    fn has_fur(&self) -> bool {
        true
    }
}

struct Cat {
    name: String,
    is_happy: bool,
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn speak(&self) -> String {
        if self.is_happy {
            String::from("meOW")
        } else {
            String::from("hisssss")
        }
    }

    fn n_legs(&self) -> u32 {
        4
    }

    fn has_fur(&self) -> bool {
        true
    }
}

struct Frog {
    name: String,
}

impl Animal for Frog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn speak(&self) -> String {
        String::from("ribbot")
    }

    fn n_legs(&self) -> u32 {
        4
    }

    fn has_fur(&self) -> bool {
        false
    }
}
