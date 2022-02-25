use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess how old Aimery is!");

    let aimery_age = rand::thread_rng().gen_range(1..101);

    println!("Aimery is {} years old", aimery_age);

    loop {    
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed that Aimery is {}", guess);

        match guess.cmp(&aimery_age) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
