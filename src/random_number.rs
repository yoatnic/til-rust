extern crate rand;
extern crate std;

use self::std::io;
use self::rand::Rng;
use std::cmp::Ordering;

pub fn random_exapmle() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        let guessed_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Plz input number");
                continue;
            },
        };;

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}