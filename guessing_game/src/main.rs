use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let target = rand::thread_rng().gen_range(1..=100);

    println!("input:");

    let mut guess = String::new();

    loop {
        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line; this line prints in the crash report");

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(err) => {
                println!("failed to parse. error: {}", err);
                continue;
            }
        };

        match guess.cmp(&target) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("winner");
                return;
            }
        }

        println!("you guessed: {guess}");
    }
}
