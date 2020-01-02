use rand::prelude::*;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let _stdin: std::io::Stdin = io::stdin(); // function returns handle to
                                              // standard input(std::io::Stdin) of teminal
    let mut guesStrBuf = String::new();
    let size = _stdin.read_line(&mut guesStrBuf); // &-reprsents a reference type
                                             // &mut -> mutable reference

    //Cargo download external dependency from crates.io and also downloads dependencies if any
    // Cargo uses semantic versioning.The number 0.3.14 is actually shorthand for ^0.3.14,
    // which means “any version that has a public API compatible with version 0.3.14.”

    //Que-Cargo downloads code or binaries -
    // what is the location of download

     let num = rand::random::<u8>();
    //let random_num_generator = rand::thread_rng();
    //let num = random_num_generator.gen::<i32>();
     let guessStr = guesStrBuf.trim();
    let input_num = guessStr.parse::<u8>();

    match input_num {
        Ok(n) => {
            if (input_num.unwrap() == num) {
                println!("You guessed it!");
            } else {
                println!("Better luck next time! Randome number is {}",num);
            }
        }
        Err(n) => println!("Error:: {}", n),
    }

    match size {
        Ok(n) => println!("Your guess is {} with size {}", guessStr, n),
        Err(n) => println!("Error {}", n),
    }
}



