use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        guess.clear();
        println!("Please guess a number between 1 and 100.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The secret number was {}", secret_number);
                break;
            }
        }
    }
}
