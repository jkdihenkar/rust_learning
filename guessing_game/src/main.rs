use std::io;

fn main() {
    println!("Guess the number name!");

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
}
