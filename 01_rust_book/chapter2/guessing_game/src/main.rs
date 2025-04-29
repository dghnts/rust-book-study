use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // macro
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
            // macro
            println!("Please unput your guess.");

            // variable
            // variable ist immutable by default
            // create mutable valiable
            // This is a String type empty string
            let mut guess = String::new(); // muttable variable

            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess: u32 = guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue;
            }
            println!("You gueessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
}
