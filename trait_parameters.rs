use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    r : f64
}

#[derive(Debug)]
struct Square {
    a : f64
}

trait Shape {
    fn area(&self) ->  f64;
    fn perimeter(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.a * self.a;
    }

    fn perimeter(&self) -> f64 {
        return 4.0 * self.a;
    }
}

impl Shape for Circle {

    fn area(&self) -> f64 {
        return 3.14 * self.r * self.r;
    }

    fn perimeter(&self) -> f64 {
        return 3.14 * 2.0 * self.r;
    }
}

fn print_info(shape : impl Shape + Debug) {
    println!("area of shape is {}", shape.area());
    println!("perimeter of shape is {}", shape.perimeter());
    println!("{:?}", shape);
}

fn print_infos<T : Shape + Debug>(shape1 : T, shape2 : T) {
    print_info(shape1);
    print_info(shape2);
}

fn print_info_where<T>(shape : T) where T:Shape + Debug {
    print_info(shape);
}


fn main() {
    let s : Square = Square{a : 5.0};
    let c : Circle = Circle{r : 3.0};
    let s1 : Square = Square{a:6.0};
    let s2 : Square = Square{a:8.0};
    println!("for square");
    print_info(s);

    println!("for circle");
    print_info(c);

    println!("for both square s1 and s2");
    print_infos(s1, s2);

    let c1 = Circle {r: 9.0};
    print_info_where(c1);
}
