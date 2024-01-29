// importing the io library from std library.
use std::io;
// importing the cmp ordering for enum of Less/Greater/Equal
use std::cmp::Ordering;
// import the rand
use rand::Rng;

// The fn syntax declares a new function;
// the parentheses, (),
// indicate there are no parameters; and the curly bracket, {, starts the body of the function.

fn main() {
    println!("Guess the number!");

    // generate secrete number of range 1-100 using rand crates!
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop creates an infinite loop, we'all add a loop to give users more chances
    loop {
        println!("Please input your guess!");

        // creating variable to store the user input.
        // variables are immutatble by default
        // which means once value given that value can not be change.
        // To make a variable mutable, we add `mut` before the variable name:

        // The :: syntax in the ::new line indicates that new is an associated function of the String type
        let mut guess = String::new();

        // references are immutable by default.
        // The & indicates that this argument is a reference
        // NOTE: which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        // let guess: u32 =  guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
        };
        
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("Yeahhh... win!");
                break;
            },
        }
    }
}

/*
RUST: println!()

    - when print the value of a variable, the variable name can go inside the curly branckets
    - when printing the result of evaluating an expression, place empty curly brackets in the format string then,
    - follow the format string with a comma-separated list of expresssion to print in each empty curly branckets

    eg. 1
     println!("You guessed : {guess}") // guess is nothing but the variable.

    eg. 2
      println!("your result is : {}", result) // result is expression evaluation.


    eg. 3
     - printing the varaible and express in one call to println!

     let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);


*/
