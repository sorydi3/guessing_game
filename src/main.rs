
use std::io;

fn main() {
    println!("GUESSING THE NUMBER GAME!!");

    println!("PLEASE ENTER THE NUMBER!");

    let mut guess : String = String::new();

    io::stdin().read_line(&mut guess).expect("I don't know");

    //let user_guess :usize = result.expect("I don't know");

    println!("you guessed the number: {guess}");

}
