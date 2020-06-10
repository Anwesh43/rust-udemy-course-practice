use std::fmt::Display;
use std::ops::Add;

struct Complex<T> {
    re : T,
    im : T
}

impl<T> Complex<T> where T : Display {
    fn new(re : T, im : T) -> Complex<T> {
        return Complex{re : re, im : im}
    }
    fn to_string(&self) {
        println!("{} + {}i", self.re, self.im);
    }
}

impl Add for Complex<i32> {

    type Output = Complex<i32>;

    fn add(self, c1 : Self) -> Self::Output {
        return Complex::new(self.re + c1.re, self.im + c1.im);
    }
}

fn main() {
    let comp1  = Complex::new(2, 3);
    let comp2 = Complex::new(3.2, 4.3);
    let comp3 = Complex::new(3, 4);
    comp1.to_string();
    comp2.to_string();
    comp3.to_string();
    let comp4 = comp1 + comp3;
    comp4.to_string();
}
