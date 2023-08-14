use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let goal = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number 1-100: ");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Reading line failed");

        let guess: u32 = match line.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Not a positive integer.");
                continue;
            }
        };

        if guess > 100 || guess == 0 {
            println!("Out of range.");
            continue;
        }

        match guess.cmp(&goal) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        };
    }
}
