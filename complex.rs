use std::fmt::Display;

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

fn main() {
    let comp1  = Complex::new(2, 3);
    let comp2 = Complex::new(3.2, 4.3);
    comp1.to_string();
    comp2.to_string();
}
