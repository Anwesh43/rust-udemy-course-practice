fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    println!("{:?}", a);
    let mut idx : usize = 0;
    println!("at 0 for a we have {}", a[idx]);
    a[idx] = 10;
    println!("at 0 for a we have {}", a[idx]);
    idx = 5;
    match a.get(idx) {
        Some(x) => println!("at {} for a we have {}", idx, x),
        None => println!("error at {} we have nothing", idx)
    };
    a.push(10);
    a.push(12);
    a.push(14);
    let k = a.pop();
    match k {
        Some(x) => println!("last element is {}", x),
        None => println!("we have no more elements in a")
    };
    println!("k is {:?} and a is {:?}", k, a);
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
    println!("{:?}", a);
}

fn main() {
    vectors();
}
