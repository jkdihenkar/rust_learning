#[derive(Copy, Clone)]
struct Number {
    odd: bool,
    value: i32,
}

fn main() {
    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: 2,
    };

    print_number(one.clone());
    print_number(two.clone());

    print_number_matcharm(one.clone());
    print_number_matcharm(two.clone());
}

fn print_number(n: Number) {
    if let Number { odd: true, value } = n {
        println!("Odd number {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number {}", value);
    }
}

fn print_number_matcharm(n: Number) {
    match n {
        Number { odd: true, value } => {
            // expression
            println!("Odd number {}", value)
        }
        Number { odd: false, value } => println!("Even number {}", value),
    }
}
