use std::mem;
fn arrays() {
    let mut a : [i32;5] = [0, 1, 2, 3, 4];
    a[0] = 10;
    println!("a has {} elements and first value is {}", a.len(), a[0]);
    for i in a.iter() {
        println!("element is {}", i);
    }
    println!("{:?}" , a);
    if a == [10, 1, 2, 3, 4] {
        println!("match");
    }
    let b = [1; 10];
    println!("{:?}", b);
    println!("size of b is {} bytes", mem::size_of_val(&b));
    let mat : [[i32;3]; 2] = [
        [0, 1, 2],
        [2, 3, 4]
    ];
    println!("{:?}", mat);
    for k in mat.iter() {
        for p in k.iter() {
            println!("matrix element is {}", p);
        }
    }
}

fn main() {
    arrays();
}
