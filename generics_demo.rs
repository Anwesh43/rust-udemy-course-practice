struct Point<T> {
    x : T,
    y : T
}

struct PointTV<T, V> {
    x : T,
    y : V
}

struct Line<T, V> {
    start : Point<T>,
    end : Point<V>
}

fn main() {
    let p1 : Point<i64> = Point {x : 1, y : 3 };
    let p2 : Point<f64> = Point {x : 3.4, y : 2.4} ;
    println!("p1 is {}, {}", p1.x, p1.y);
    println!("p2 is {}, {}", p2.x, p2.y);

    let p3 : PointTV<i64, f64> = PointTV {x : 1, y : 3.2};
    println!("p3 is {}, {}", p3.x, p3.y);

    let line : Line<i64, f64> = Line{start : p1, end : p2};
    println!("line' is from ({},{}) to ({}, {})", line.start.x, line.start.y, line.end.x, line.end.y);
}
