use std::io::stdin;

fn main() {
    let number = 34;
    println!("enter your guess");
    loop {
        let mut buffer  = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess >= 101 || guess < 1 {
                            println!("number is out of bounds");
                            continue;
                        }
                        if guess < number {
                            println!("number is too low");
                        } else if guess > number {
                            println!("number is too high");
                        } else {
                            println!("you guessed the right number");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("enter a valid number between 1 to 100");
                    }
                }

            },
            Err(_) => {
                println!("enter a valid number between 1 to 100");
            }
        }
    }
}
