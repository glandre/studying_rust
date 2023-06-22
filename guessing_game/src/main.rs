use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("");
    println!("");
    println!("A secret number was generated, can you guess it?");
    
    let mut num_attempts = 10;
    println!("");
    println!("You have {num_attempts} attempts...");
    println!("");
    println!("");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("");
        println!("You guessed: {guess}");
        println!("");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        println!("");
        println!("");

        num_attempts -= 1;

        if num_attempts > 0 {
            println!("Number of attempts: {num_attempts}");    
        } else {
            break;
        }
        
    }
}
