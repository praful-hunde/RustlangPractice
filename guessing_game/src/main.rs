use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your gess");

    let mut guess = String::new(); //The :: syntax in the ::new line indicates that new is an associated function of the String type.
                                   //In other words new is a static function of String type.

    let std_in: io::Stdin = io::stdin(); // The function stdin from io module retruns a handle to standard input (io::Stdin)
    let result: io::Result<usize> = std_in.read_line(&mut guess); //& indicates a reference type. References are immutable by default , thus &mut is passed.
    let string_size: usize = result.expect("Failed to read line"); // expect let your program crash on error with given message
                                                                   // and return value of function if reuslt is ok.

    // match result {
    //     Ok(str_size) => str_size,
    //     Err(err) => panic!("Failed to read line {}",err)
    // }

    if string_size == guess.len() {
        println!("Input string length is same as value of size retured by read_line.");
    }
    /*  Above three lines can be written as below
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    */
    println!("You gessed : {}", guess );
    println!("You gessed : {}", guess );

}
