#[allow(dead_code)]
enum Color{
    GREEN,
    RED,
    BLUE,
    PURPLE,
    RGB(u8, u8, u8),
    CYMK{cyan : u8, yellow : u8, magenta: u8, black: u8}
}

fn main() {
    let c : Color = Color::CYMK{cyan :100, yellow : 50, magenta : 78, black : 10};
    let a = match c {
        Color::RED => "red",
        Color::GREEN => "green",
        Color::BLUE => "blue",
        _ => "Any color"
    };

    match  c {
        Color::RED => println!("red"),
        Color::GREEN => println!("green"),
        Color::BLUE => println!("blue"),
        Color::RGB(0, 0, 0) => println!("black"),
        Color::RGB(r, g, b) => println!("{}, {}, {}", r, g, b),
        Color::CYMK{cyan : c, yellow : y, magenta : m, black : b} => println!("{}, {}, {}, {}", c, y, m, b),
        _ => println!("Any Color")
    }

    println!("a is {}", a);
}
