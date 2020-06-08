use std::collections::HashMap;

struct Dog {
    name : &'static str,
    sound : &'static str
}

trait Animal {
    fn say(&self);
}

impl Animal for Dog {
    fn say(&self) {
        println!("my name is {} and I {}", self.name, self.sound);
    }
}

fn main() {
    let dog = Dog{name : "Barne", sound : "Bark"};
    dog.say();
    let mut hm = HashMap::new();
    let mut ve = Vec::new();
    hm.insert("a", "1");
    match hm.get("a") {
        Some(a) => println!("we got {}", a),
        None => println!("We got none")
    }
    ve.push("a");
    ve.push("b");
    ve.push("c");
    let k = ve.pop();
    match k {
        Some(num) => println!("number i popped {}", num),
        None => println!("none")
    }
}
