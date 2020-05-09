fn if_is_demo(temp : i32) {
    if temp > 30 {
        println!("too hot");
    } else if temp < 20 {
        println!("too cold");
    } else {
        println!("normal room temperature");
    }
}

fn check_temp_day(temp : i32) {
    let day = if temp > 30 {"sunny"} else {"clear"};
    println!("day is {}", day);
}

fn main() {
    if_is_demo(32);
    if_is_demo(8);
    if_is_demo(23);
    check_temp_day(32);
    check_temp_day(22);

}
