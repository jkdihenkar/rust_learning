
fn main() {
    // playing with scopes and blocks
    let x = "out";
    println!("{}", x);
    {
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);

    // blocks are also expressions and can be evaluated to a value

    // example - 1
    let k = {
        7
    };
    println!("k={}", k);

    // example - 2
    let x = {
        let x = 27;
        let y = 3;
        y+x
    };
    println!("x={}", x);

    // function definitions and blocks - example 1

    fn fair_roll_dice_v1() -> i8 {
        4
    }

    fn fair_roll_dice_v2() -> i8 {
        return 7;
    }

    println!("v1={} and v2={}", fair_roll_dice_v1(), fair_roll_dice_v2());

    // if conditions are also expressions - example 1
    let feeling_luck = false;

    fn fair_roll_dice_v3(feeling_luck: bool) -> i8 {
        if feeling_luck {
            6
        } else {
            4
        }
    }

    println!("v3({})={}", feeling_luck, fair_roll_dice_v3(feeling_luck));

    // match is also an expression - example 1
    fn fair_roll_dice_v4(feeling_luck: bool) -> i8 {
        match feeling_luck {
            true => 4,
            false => 6,
        }
    }

    println!("v4({})={}", feeling_luck, fair_roll_dice_v4(feeling_luck));
    
    // usgae of dots - example 1
    let a = ("some", 'r', 227);
    println!("a(0,1,2) = ( {}, {}, {} )", a.0, a.1, a.2);
    // dots can also be used to call a function of an element
    println!("len(a.0) = {}", a.0.len());

    // similar to dots - double colon :: can be used for namespaces
    // std is a crate (like a library) ; cmp is a module (source file) ; min() is a function
    let min_n = std::cmp::min(3, 6);
    println!("min(3,6) = {}", min_n);

    // use directive can be used to bring in scope the names from other namespaces
    // !can also be used inside functions and blocks
    use std::cmp::min;
    println!("min(4,99) = {}", min(4, 99));

    // types are namespaces too and methods can also be called regularly
    let x = "something";
    println!("len({}) = {}", x, str::len(x));

    // structs
    struct StrucExample {
        first_int: i32,
        second_float: f64,
        third_bool: bool
    }

    // when init - order doesn't matter only the names do
    let struc_1 = StrucExample {first_int: 23, second_float: 445.678, third_bool: true};
    println!("struc_1.second_float = {}", struc_1.second_float);
    println!(
        "struc_1 = first_int: {}, second_float: {}, third_bool: {}", 
        struc_1.first_int, struc_1.second_float, struc_1.third_bool
    );

    // shortcut for init the rest of the fields from other struct
    let struc_2 = StrucExample { third_bool: false, ..struc_1};
    println!(
        "struc_2 = first_int: {}, second_float: {}, third_bool: {}", 
        struc_2.first_int, struc_2.second_float, struc_2.third_bool
    );

    // structs can also be deconstructed like tuples
    let struc_3 = StrucExample {..struc_2};
    // struct field names are now mandatory as opposed to shown in old examples
    let StrucExample {first_int: a, second_float: b, third_bool: c} = struc_3;
    println!("a={}, b={}, c={}", a, b, c);

    // let patterns can be used as conditions and also in match arm
    struct Number {
        odd: bool,
        value: i32
    }

    let one = Number {odd: true, value: 1};
    let two = Number {odd: false, value: 2};
    let three = Number {odd: true, value: 3};
    let four = Number {odd: false, value: 4};

    // let as conditions in if
    fn print_number_v1(n: Number) {
        if let Number {odd: true, value: v} = n {
            println!("odd Number {}", v);
        } else if let Number {odd: false, value: v} = n {
            println!("even Number {}", v);
        }
    }

    print_number_v1(one);
    print_number_v1(two);

    // conditions in match arm
    fn print_number_v2(n: Number) {
        match n {
            Number {odd: true, value: v} => println!("Odd Number - {}", v),
            Number {odd: false, value: v} => println!("Even Number = {}", v)
        }
    }

    print_number_v2(three);
    print_number_v2(four);

    // match arms have to be exhaustive else we'd get a compile time error
    fn print_number_v3(n: Number) {
        match n {
            Number {value: 1, ..} => println!("One"),
            Number {value: 2, ..} => println!("Two"),
            Number {value, ..} => println!("{}", value)
        }
        // error[E0004]: non-exhaustive patterns: `Number { value: i32::MIN..=0_i32, .. }` 
            // and `Number { value: 3_i32..=i32::MAX, .. }` not covered
    }

    // if that's hard underscores can also be used as 'catch-all'
    fn print_number_v4(n: Number) {
        match n.value {
            1 => println!("One"),
            2 => println!("Two"),
            _ => println!("{}", n.value)
        }
    }

    // have to init these once again coz their values are moved
    let one = Number {odd: true, value: 1};
    let two = Number {odd: false, value: 2};
    let three = Number {odd: true, value: 3};
    let four = Number {odd: false, value: 4};

    print_number_v3(one);
    print_number_v4(two);
    print_number_v3(three);
    print_number_v4(four);

    // can declare method on once's own types
    impl Number {
        fn is_positive(self) -> bool {
            self.value > 0
        }
    }

    let minus_one = Number {odd: true, value: -1};

    println!("is minus_one positive? - {}", minus_one.is_positive());

    // mut makes the variable bindings mutable
    let _x = 37;
    // x = 42;
    // 189 |     x = 42;
    //     |     ^^^^^^ cannot assign twice to immutable variable
    let mut x = 32;
    println!("x = {}", x);
    x = 47;
    println!("x = {}", x);

}
