fn arrays() {
    let mut a : [i32;5] = [0, 1, 2, 3, 4];
    a[0] = 10;
    println!("a has {} elements and first value is {}", a.len(), a[0]);
}

fn main() {
    arrays();
}
