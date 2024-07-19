// standalone example: tuples.rs

// ❯❯❯ rustc tuples.rs -o bin/tuples ; ./bin/tuples
// Pair = (a, 17)
// somechar=b, someint=19

fn main() {
    // basic tuples
    let pair = ('a', 17);
    println!("Pair = ({}, {})", pair.0, pair.1);

    // destructuring tuples
    let (somechar, someint) = ('b', 19);
    println!("somechar={}, someint={}", somechar, someint);

    // splits and discarding values
    let (_, right) = "something_x_new".split_at(11);
    println!("right of (something_x_new) is {}", right);
}
