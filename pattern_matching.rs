fn match_number(x : i32) -> &'static str {
    return match x {
        0 => "no",
        1 | 2 => "one or two",
        9...11 => "lot of",
        _ if (x % 2 == 0) => "some of",
        _ => "few of"
    };
}

fn match_points(point : &(i32, i32)) -> &'static str {
    return match point {
        (0, 0) => "origin",
        (_, 0) => "on y axis",
        (0, _) => "on x aixs",
        (_, _) => "at x y plane"
    };
}

fn main() {
    println!("we are matching numbers");
    println!("______________________=>");
    for i in 0..13  {
        println!("we have {}  oranges", match_number(i));
    }

    let points : [(i32,i32);4] = [(0, 0), (2, 0), (0, 2), (2, 2)];
    println!("__________XXX___________");

    println!("we are matching tuples");
    println!("______________________=>");
    for point in points.iter() {
        println!("we have {}", match_points(point));
    }
    println!("__________XXX___________");
}
