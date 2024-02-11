fn main() {
    let number = 2;

    // blocks of code associated with conditions in `if` expressions are sometimes called `arms` like the arms in `match`
    // note condition in `if` must be a `bool` else it's an error.
    if number < 3 {
        println!("Condition was TRUE!");
    } else {
        println!("condition was FALSE!");
    }

    // Rust will not automatically try to convert non-Boolean types to Boolean.

    let number = 3;

    if number != 0 {
        println!("Number is other than zero!");
    }

    // we can also make use of `if ... else.if... else`
    // if ... else.if ... else can clutter the code so to refactor we can use `match` expression in rust

    // Using `if` in `let` statement

    let condition = true;

    let magic_number = if condition { 5 } else { 3 };
    println!("Magic number goes here {magic_number}");
    
}
