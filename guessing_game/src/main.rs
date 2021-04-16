use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Input your guess: ");

    // variables are by default immutable
    // so mut makes it mutable
    let mut guess = String::new();

    io::stdin()
        // The & indicates that this argument is a reference
        //  like variables, references are immutable by default.
        //  Hence, you need to write &mut guess rather than &guess to make it mutable.
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("The number you guessed is :: {}", guess);

    let guess_int: u32 = guess.trim().parse().expect("Please type a number!");

    match guess_int.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
