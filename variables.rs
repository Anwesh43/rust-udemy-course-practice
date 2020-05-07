use std::mem;
fn main() {
    let x : i64 = 200;
    println!("x is {}", x);
    let x : f32 = 3.4;
    {
        let x : char = 'a';
        println!("x is {}", x);
    }
    println!("x is {}", x);
    let y = Box::new(x);
    println!("y is {}", y);
    println!("size of y {}", mem::size_of_val(&y));
    let k = *y;
    println!("value of k {} size of k {}", k, mem::size_of_val(&k));
}
