fn sum_and_product(a : i32, b : i32) -> (i32, i32) {
    return (a + b, a * b);
}

fn main() {
    let sp  : (i32, i32) = sum_and_product(10, 20);
    println!("{:?}", sp);
    println!("for 10 and 20, sum is {} and product is {}", sp.0, sp.1);
    let (a, b) = sp;
    println!("a is {} and b is {}", a, b);
    let sp1 : (i32, i32) = sum_and_product(5, 10);
    let spsp1 : ((i32, i32), (i32, i32))= (sp, sp1);
    println!("{:?}", spsp1);
    let ((a1, a2), (a3, a4)) = spsp1;
    println!("a1 is {}, a2 is {}, a3 is {}, a4 is {}", a1, a2, a3, a4);
    println!("value of a1 is {}", (spsp1.0).0);
    let k = (42, );
    println!("{:?}", k);
}
