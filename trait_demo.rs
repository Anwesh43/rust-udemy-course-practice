trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rect {
    l : f64,
    h : f64
}

impl Rect {

    fn rect_str(&self) {
        println!("the rectangle's length is {} and height is {}", self.l, self.h);
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        return self.l * self.h;
    }
    fn perimeter(&self) -> f64{
        return 2.0 * (self.l + self.h);
    }
}

fn main() {

    let r : Rect = Rect {l : 10.0, h : 20.0};
    r.rect_str();
    println!("Area of rect is {}", r.area());
    println!("Perimeter of rect is {}", r.perimeter());
}
