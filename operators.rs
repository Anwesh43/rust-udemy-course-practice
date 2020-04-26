fn main() {
    //cubes
    let a : i32 = 8;
    let a_cubed : i32 = i32::pow(a, 3);
    println!("a is {} a_cubed is {}", a, a_cubed);
    let b : f64 = 4.5;
    let b_cubed : f64 = f64::powi(b, 3);
    let b_24 : f64 = f64::powf(b, 2.4);
    println!("b_cubed = {}, b_24 = {}", b_cubed, b_24);
    let pi314 : bool = std::f64::consts::PI < 4.0;
    println!("PI less than 4 = {}", pi314);
    let two_to_10 = 1 << 10;
    println!("2 to the power 10 = {}", two_to_10);
}
