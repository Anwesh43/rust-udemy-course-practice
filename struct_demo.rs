struct Point {
    x : i64,
    y : i64
}

struct Line {
    p1 : Point,
    p2 : Point
}

fn square(a : i64) -> i64 {
    return a * a
}

fn display_line(l : Line) {
    println!("1:{},{} 2:{}, {}", l.p1.x, l.p1.y, l.p2.x, l.p2.y);
}

fn main() {
    let origin : Point = Point {x : 0, y : 0};
    let p1 : Point = Point {x : 5, y : 5};
    println!("{}, {}", origin.x, origin.y);
    let dsqr : i64 = square(p1.x - origin.x) + square(p1.y - origin.y);
    println!("distance is {}", dsqr);
    let line : Line = Line {p1 : p1, p2 : origin};
    display_line(line);
}
