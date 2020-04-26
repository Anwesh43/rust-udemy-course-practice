use std::mem;

fn main() {
    let a = 344234321;
    println!("a = {} and sizeof a is {}",a, mem::size_of_val(&a));
    let a = "hello";
    println!("a = {} and sizeof a is {}", a, mem::size_of_val(&a));
    let b : isize = 123456789;
    let c = mem::size_of_val(&b);
    println!("b = {}, memory = {}", b, c);
    let d : char = 'x';
    println!("d = {}, size of d = {}", d, mem::size_of_val(&d));
    let e : bool = true;
    println!("e = {}, size of e = {}", e, mem::size_of_val(&e));
}
