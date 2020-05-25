fn closures() {
    let hs = |x : i32| -> i32 {x + 1};
    println!("{}", hs(6));
    let inc = |x : &mut i32| {*x += 3;};
    let mut f = 12;
    inc(&mut f);
    println!("new value of f is {}", f);
}

fn main() {
    closures();
}
