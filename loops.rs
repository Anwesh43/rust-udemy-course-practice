fn while_loop(n : i64) {
    let mut i : i64 = 0;
    while i < n {
        println!("i is {}", i);
        i = i + 1;
    }
}

fn loop_demo(n : i64) {
    let mut i : i64 = 0;
    loop {
        i = i + 1;
        println!("i is {}", i);
        if i == n {
            break;
        }
    }
}

fn for_loop(n : i64) {
    for i in 1..n {
        println!("i is {}", i * 100);
    }
}

fn for_loop_enumerate(a : i64, b : i64) {
    for (key, value) in (a..b).enumerate() {
        println!("{}:{}", key, value);
    }
}

fn main() {
    while_loop(10);
    loop_demo(10);
    for_loop(11);
    for_loop_enumerate(30, 40);
}
