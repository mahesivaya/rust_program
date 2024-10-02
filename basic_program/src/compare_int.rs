use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn compare_int() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // Handling Invalid input value and keeping asking guess until it wins
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };



        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // Below both Ordering do the same task
            // Ordering::Equal => println!("You win!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}