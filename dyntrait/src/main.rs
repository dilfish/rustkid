// rust must return a constant memory object
// so a trait is not valid return type
// we could return a Box

// sean at shanghai
// 2022

struct Sheep{}
struct Cow{}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep{})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("you have randomly chosen an animal, and it says {}", animal.noise());
}
