
use std::io;

fn main() {
    println!("GUESSING THE NUMBER GAME!!");

    println!("PLEASE ENTER THE NUMBER!");

    let mut guess : String = String::new();

    let result:Result<usize ,io::Error> =io::stdin().read_line(&mut guess);

    if result.is_ok() {
        println!("you guessed the number: {guess}");
    }

    if result.is_err() {
        println!("somthing when wrong while reading the user guess!!!")
    }
    //let user_guess :usize = result.expect("I don't know");
}
