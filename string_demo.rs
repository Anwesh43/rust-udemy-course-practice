fn main() {
    let s : &'static str = "hello world";
    println!("{}", s);
    println!("printing all characters =========");
    for c in s.chars() {
        println!("{}", c);
    }
    println!("printing all characters in reverse=======");
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first character of {} is {}", s, first_char);
    }


    if let Some(hundreth_char) = s.chars().nth(100) {
        println!("first character of {} is {}", s, hundreth_char);
    } else {
        println!("no 100th character exists");
    }

    println!("creating a new string");

    let mut new_str = String::new();
    let mut a  = 'a' as u8;

    while a <= 'z' as u8 {
        new_str.push(a as char);
        if a != 'z' as u8 {
            new_str.push_str(", ");
        }
        a =  a + 1;
    }

    println!("new string is {}", new_str);
    let concat_str =  String::from("hello") + " world";
    let u:&str = &new_str;
    println!("{}", u);

    println!("{}", concat_str);
    let mut hello_world_string = String::from(s);
    hello_world_string.remove(0);
    hello_world_string.push_str("! Good Morning");
    println!("{}", hello_world_string.replace("ello", "Hola"));
}
