use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing Game 0-100!");


    let secret = rand::thread_rng().gen_range(0, 100);
    println!("Please input your guess!");

    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("You win! {} is the secret number!", secret);
                break;
            },
        }
    }
}