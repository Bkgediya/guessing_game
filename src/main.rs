use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");
    let secret_number= rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed t  o read line");

        let guess: u32  = match  guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue 
        };

        println!("You guess {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Win");
                break;
        }
    }

    println!("You guess {}",guess)
}

}
