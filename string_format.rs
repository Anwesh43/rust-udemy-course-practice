fn main() {
    let hello = "hello";
    let world = "world";
    let hello_world = format!("{} {}!", hello, world);
    println!("{}", hello_world);

    let more_hello_world = format!("{0} {1} and {0} {1}", hello, world);
    println!("{}", more_hello_world);

    let hello_world_again = format!("{hello} {world} {} {1} {2}", "and", hello = hello, world = world);
    println!("{}", hello_world_again);
}
