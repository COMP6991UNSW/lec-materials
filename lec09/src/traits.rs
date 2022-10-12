struct BestFriends<A1, A2> {
    animal1: A1,
    animal2: A2,
}

impl<A1, A2> BestFriends<A1, A2>
where
    A1: Animal,
    A2: Animal,
{
    fn both_greet(&self) -> String {
        format!(
            "{}\n{}",
            self.animal1.speak(),
            self.animal2.speak()
        )
    }

    fn say_hello_to_each_other(&self) -> String {
        format!(
            "{}\n{}",
            self.animal1.say_hello_to(&self.animal2),
            self.animal2.say_hello_to(&self.animal1)
        )
    }
}

fn all_animals_greet(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("{}", animal.speak());
    }
}

pub fn main() {
    all_animals_greet(vec![
        Box::new(Cow {
            name:     String::from("Shaun"),
            age:      4,
            angry:    false,
        }),
        Box::new(Sheep {
            name:     String::from("Miguel"),
            age:      4,
            at_party: true,
        }),
        Box::new(Sheep {
            name:     String::from("Alex"),
            age:      4,
            at_party: true,
        }),
    ]);


    let best_friends = BestFriends {
        animal1: Sheep {
            name:     String::from("Shaun"),
            age:      4,
            at_party: true,
        },
        animal2: Cow {
            name:  String::from("Betsie"),
            age:   10,
            angry: false,
        }
    };

    println!("{}", best_friends.both_greet());
    println!("{}", best_friends.say_hello_to_each_other());
}








trait Animal {
    fn name(&self)  -> String;
    fn age(&self)   -> u32;
    fn speak(&self) -> String;

    fn say_hello_to<A>(&self, animal: &A) -> String
    where
        A: Animal,
        Self: Sized;
}









impl Animal for Sheep {
    fn name(&self)  -> String {
        self.name.clone()
    }

    fn age(&self)   -> u32 {
        self.age
    }

    fn speak(&self) -> String {
        if self.at_party {
            String::from("Ba tss Ba tss Ba tss 😎")
        } else {
            String::from("baAaAaAaAaAa")
        }
    }

    fn say_hello_to<A>(&self, animal: &A) -> String
    where
        A: Animal,
    {
        if self.at_party {
            format!("how's the baAaAaArty, {}! 🎉🎉🎉", animal.name())
        } else {
            format!("baAaAa to you, {}", animal.name())
        }
    }
}






impl Animal for Cow {
    fn name(&self)  -> String {
        self.name.clone()
    }

    fn age(&self)   -> u32 {
        self.age
    }

    fn speak(&self) -> String {
        if self.angry {
            String::from("MOOOOOOOOO")
        } else {
            String::from("moooOOOooo")
        }
    }

    fn say_hello_to<A>(&self, animal: &A) -> String
    where
        A: Animal,
    {
        if self.angry {
            format!("MOO YOU, {}", animal.name())
        } else {
            format!("mooouwu, {} :3", animal.name())
        }
    }
}

impl Animal for Crab {
    fn name(&self)  -> String {
        String::from("ferris")
    }

    fn age(&self)   -> u32 {
        self.age
    }

    fn speak(&self) -> String {
        String::from("*crab noises*")
    }

    fn say_hello_to<A>(&self, animal: &A) -> String
    where
        A: Animal,
    {
        format!("attacks {} with claws", animal.name())
    }
}









struct Sheep {
    name:     String,
    age:      u32,
    at_party: bool,
}

struct Cow {
    name:  String,
    age:   u32,
    angry: bool,
}

struct Crab {
    age: u32,
}










