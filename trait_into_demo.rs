use  std::convert::Into;
struct Creature {
    name : String
}

struct Person {
    name : String
}

impl Person {
    fn new<S: Into<String>>(name : S) -> Person {
        return Person {name : name.into()};
    }

}

impl Into<Creature> for Person {
    fn into(self) -> Creature {
        return Creature{name : self.name};
    }
}

impl Creature {
    fn new<S: Into<Creature>>(c : S) -> Creature {
        return Creature{name : c.into().name};
    }
}


fn main() {
    let p = Person::new("ADemo");
    println!("using into");
    println!("person is {}", p.name);
    println!("creating a custom into implementation");
    let c : Creature = Creature::new(p);
    println!("creature name is {}", c.name);
}
