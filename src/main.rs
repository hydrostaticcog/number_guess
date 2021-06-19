use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Number Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read the line!");
        println!("You guessed {}", guess);

        let guess: u32 = guess.trim().parse().expect("You need to input an integer!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small!"),
            Ordering::Greater => println!("Your guess was too large!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                println!("You have won this game. Congratulations!");
                break;
            }
        }
    }
}
