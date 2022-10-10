pub fn main() {
    let shaun = Sheep {
        name:     String::from("Shaun"),
        age:      4,
        at_party: true,
    };

    let betsie = Cow {
        name:  String::from("Betsie"),
        age:   10,
        angry: false,
    };

    println!("{}", shaun.speak());
    println!("{}", betsie.speak());
    println!("{}", shaun.say_hello_to(&betsie));
    println!("{}", betsie.say_hello_to(&shaun));
}









trait Animal {
    fn name(&self)  -> String;
    fn age(&self)   -> u32;
    fn speak(&self) -> String;

    fn say_hello_to<A>(&self, animal: &A) -> String
    where
        A: Animal;
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
