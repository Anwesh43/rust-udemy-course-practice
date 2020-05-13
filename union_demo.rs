union IntOrFloat {
    i : i32,
    f : f32
}

fn check_value(un : IntOrFloat) {
    unsafe {
        match un {
            IntOrFloat {i : 5} => println!("value is {}", un.i),
            IntOrFloat {f} => println!("float value is {}", un.f)

        }
    }

}

fn main() {
    let un = IntOrFloat {i : 5};
    unsafe {println!("value is {}", un.i)};
    check_value(un);
    let un1 = IntOrFloat {f : 2.0};
    check_value(un1);
}
