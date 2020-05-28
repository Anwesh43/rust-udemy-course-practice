struct Human {
    name:&'static str,
    language:&'static str
}

struct Cat {
    name:&'static str,
    sound:&'static str
}

trait Animal {
    fn create(name:&'static str) -> Self;

    fn name(&self);

    fn talk(&self) {
        println!("cannot talk");
    }
}



impl Animal for Human {
    fn create(name : &'static str) -> Human {
        return Human{name : name, language : "English"};
    }

    fn name(&self) {
        println!("My name is {}", self.name);
    }
    fn talk(&self) {
        println!("I speak in {}", self.language);
    }
}

impl Animal for Cat {
    fn create(name : &'static str) -> Self {
        return Cat{name : name, sound : "meow"};
    }
    fn name(&self) {
        println!("I am a kitty whose name is {}", self.name);
    }
    fn talk(&self) {
        println!("I do {}", self.sound);
    }
}

fn main() {
    let h : Human = Human{name : "John", language : "English"};
    h.name();
    h.talk();
    let c : Cat = Cat {name : "Misty", sound : "meow"};
    c.name();
    c.talk();
    let h1 : Human = Animal::create("Ap3");
    h1.name();
    h1.talk();
    let c1 : Cat = Cat::create("Ap4");
    c1.name();
    c1.talk();

}
