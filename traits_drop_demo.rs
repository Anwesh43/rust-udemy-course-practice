struct Animal {
    name : String
}

impl Drop for Animal {
    fn drop(&mut self) {
        println!("dropping Animal");
    }
}

fn main() {
    let animal_str = "Tiger";
    let animal = Animal{name : animal_str.to_string()};
    drop(animal);
}
