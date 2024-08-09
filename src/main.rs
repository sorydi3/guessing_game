
use std::io;

use rand::{thread_rng, Rng};

use std::cmp::Ordering;

fn main() {
    println!("GUESSING THE NUMBER GAME!!");

    println!("PLEASE ENTER THE NUMBER!");

    let secret_number:u32 = thread_rng().gen_range(1..100);
    println!("somthing migh have been when wrong!!! SECRET NUMBER: {secret_number}");  
    
    loop {

        let mut guess : String = String::new();
        
        io::stdin().read_line(&mut guess).expect("I'expext the user input!!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err (err) => {
                println!("PARSING FILED: {err}  THE FOLLOWING DOGIT: {guess}");
                continue;
            }
        };
        
        match guess.cmp(&secret_number){
            Ordering::Equal =>  {println!("You win, both numbers match"); break;},
            Ordering::Greater => println!("Your guess is higher"),  
            Ordering::Less => println!("Your guess is lesser"),
        }
    }
    
    
    
}
