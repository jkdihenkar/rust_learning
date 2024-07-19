fn main() {
    // blocks are expressions and have implicit returns
    fn greeter(name: &str) -> String {
        format!("Greetings, {}!", name)
    }

    println!("{}", greeter("Jay"));

    // blocks as expressions
    let x = {
        let y = 32;
        let z = 64;
        z + y
    };

    println!("Value of x is: {}", x);
}
