use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

   
    let mut parse_res: Result<u32, std::num::ParseIntError>;

    //The :: syntax in the ::new line indicates that new is an associated function of the String type.
    //In other words new is a static function of String type.
    let std_in: io::Stdin = io::stdin(); // The function stdin from io module retruns a handle to standard input (io::Stdin)
    loop {
        let mut str_guess = String::new();
        let result: io::Result<usize> = std_in.read_line(&mut str_guess); //& indicates a reference type. References are immutable by default , thus &mut is passed.
        let input_str_len: usize = result.expect("Failed to read line"); // expect let your program crash on error with given message
                                                                         // and return value of function if reuslt is ok.
                                                                         // match result {
                                                                         //     Ok(str_size) => str_size,
                                                                         //     Err(err) => panic!("Failed to read line {}",err)
                                                                         // }
        let guess_str_len = str_guess.len();

        if input_str_len == guess_str_len {
            println!("Input string length is same as size returned by read_line.");
        }

        /*  Above three lines can be written as below
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        */
        println!("You gessed : {}", str_guess);

        parse_res = str_guess.trim().parse::<u32>();

        if parse_res.is_err() {
            println!("Not a number! Try with valid number.");
            continue;
        }

        let guessed_int: u32 = parse_res.unwrap();
        let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

        match guessed_int.cmp(&secret_number) {
            Ordering::Less => println!(
                "Guess {} is small than {}! Try Again.",
                guessed_int, secret_number
            ),
            Ordering::Greater => println!(
                "Guess {} is bigger than {}! Try Again.",
                guessed_int, secret_number
            ),
            Ordering::Equal => {println!("Guess {} is perfect! You Win.", guessed_int); break;},
        }
    }
}
