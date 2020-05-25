fn add(n : i32) -> impl Fn(i32) -> i32 {
    return move |x : i32| -> i32 { x + n};
}

fn operate(num : i32, operation : impl Fn(i32) -> i32) -> i32 {
    return operation(num);
}


fn main() {
    let add10 = add(10);
    let is_even = |x : i32| x % 2 == 0;
    let square = |x| {x * x};
    let limit = |x| {x < 500};
    println!("10 + 2 = {}", add10(2));
    println!("10 + 100 = {}", add10(100));
    println!("square of 10 is {}", operate(10, square));
    println!("30 is odd? {}", is_even(30));
    let sum2 = (0..).map(square).take_while(|x:&i32|  {limit(*x)}).filter(|x:&i32| is_even(*x)).fold(0, |sum, x| {sum + x});
    println!("sum of squares less than 500 {}", sum2);
}
