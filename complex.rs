use std::fmt::Display;
use std::ops::{Add, Neg};

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

impl<T> Add for Complex<T> where T : Add<Output = T> + Display {

    type Output = Complex<T>;

    fn add(self, c1 : Self) -> Self::Output {
        return Complex::new(self.re + c1.re, self.im + c1.im);
    }
}

impl<T> Neg for Complex<T> where T : Neg<Output=T> + Display {
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        return Complex::new(self.re.neg(), self.im.neg());
    }
}

fn main() {
    let comp1  = Complex::new(2, 3);
    let comp2 : Complex<f32> = Complex::new(3.2, 4.3);
    let comp3 = Complex::new(3, 4);
    let comp4 : Complex<f32> = Complex::new(4.9, 4.2);
    comp1.to_string();
    comp2.to_string();
    comp3.to_string();
    let comp13 = comp1 + comp3;
    comp13.to_string();
    let comp24 = comp2 + comp4;
    comp24.to_string();
    comp24.neg().to_string();
}
