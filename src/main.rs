use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("simple rustiot Game!");
    let num = rand::thread_rng().gen_range(1..=100); // Generate a random number
    loop {
        println!("Guess A number");
        let mut input = String::new(); // Create a mutable string
        io::stdin().read_line(&mut input).unwrap(); // Read user input
        let input :i32 = input.trim().parse().unwrap();
        println!("You guessed: {}", input);
        match input.cmp(&num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}