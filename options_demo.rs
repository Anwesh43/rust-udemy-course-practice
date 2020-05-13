fn main() {
    let x = 3.0;
    let y = 2.0;
    let result = if y == 0.0 {None} else {Some(x / y)};
    match result {
        Some(z) => println!("we got is {}", z),
        None => println!("Nothing")
    }
    if let Some(z) = result {
        println!("result is {}", z);
    }
}
