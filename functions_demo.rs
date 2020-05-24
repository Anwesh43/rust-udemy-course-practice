fn increase(x : &mut i32) {
    *x += 1;
}
fn print_value(x : i32) {
    println!("{}", x);
}

fn product(x : i32, y : i32) -> i32 {
    return x * y;
}

fn functions() {
    print_value(33);
    let mut z : i32 = 10;
    increase(&mut z);
    print_value(z);
    print_value(product(10, 20));
}

fn main() {
    functions();
}
