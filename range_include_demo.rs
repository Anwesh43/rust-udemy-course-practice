fn main() {
    println!("to include 1 to 10");
    for i in 1..=10 {
        println!("i is {}", i);
    }

    println!("________________>");
    println!("to not include 10");
    for i in 1..10 {
        println!("i is {}", i);
    }
}
