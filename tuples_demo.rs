fn sum_and_product(a : i32, b : i32) -> (i32, i32) {
    return (a + b, a * b);
}

fn main() {
    let sp  : (i32, i32) = sum_and_product(10, 20);
    println!("{:?}", sp);
    println!("for 10 and 20, sum is {} and product is {}", sp.0, sp.1);
}
