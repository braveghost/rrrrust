use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("guess number");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("input your guess: ");
        let mut guess_str = String::new();
        let res = io::stdin().read_line(&mut guess_str).expect("Failed to read line");
        println!("you input status: {}", res);

        println!("you guessed: {}", guess_str);
        let guess: u64 = match guess_str.trim().parse() {
            Ok(num) => num,

            Err(x) => {
                println!("input error {}", x);
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less =>
                println!("small"),
            Ordering::Greater =>
                println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }
    }
}
