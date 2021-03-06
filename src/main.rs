extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let (default_guess_limit, default_minimum_number, default_maximum_number) = (5, 1, 101);

    println!("Welcome to the awesome Guess the number game!");

    let mut guess_limit = String::new();
    println!("How many retries you want to have?");
    io::stdin().read_line(&mut guess_limit)
        .ok()
        .expect("Failed to read line. :(");

    let guess_limit = match guess_limit.trim().parse() {
        Ok(num) => {
            println!("OK! Your limit is {} guesses.", num);
            num
        },
        Err(_) => {
            println!("Invalid guesses limit. We set it to {} for you!", default_guess_limit);
            default_guess_limit
        }
    };

    println!("Now guess the number!");

    let secret_number = rand::thread_rng().gen_range(default_minimum_number, default_maximum_number);

    let mut guess_count: u8 = 0;

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line. :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        guess_count += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => { 
                println!("You win! Bravo!");
                println!("You did it with {} guesses!", guess_count);
                return;
            }
        }

        if guess_count == guess_limit {
            println!("You've reached your limit of {} tries. :(", guess_limit);
            println!("The number was {}.", secret_number);
            return;
        }
    }

}
