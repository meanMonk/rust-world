// Functions
// main is entry point for the programs.
// `fn` is keyword which allows you to declare new functions.
// - Rust code uses snake case as the conventional style for function and variable names
// - All letters are lowercase and underscores separate words
// - In function signatures, you must declare the type of each parameter
/*
    Requiring type annotations in function definitions means the compiler
    almost never needs you to use them elsewhere in the code to figure out what type you mean
    The compiler is also able to give more helpful error messages if it knows what types the function expects
*/
fn main() {
    println!("Hello, world!");

    another_function(5);
    print_value_units(2, 's');
    
    /* 
        @note : POST_IDEA
        Rust is an expression-based lang.
        Other languages don’t have the same distinctions, 
        so let’s look at what statements and expressions are and 
        how their differences affect the bodies of functions
        
        
        Statements: 
            -  instructions that perform some action and do not return a value
        Expressions:
            -  evaluate to a resultant value
            - calling function is expression
            - calling macros is express
            - 6 + 6 is expression.
            - A new scope block created with curly brackets is an expression
        
     */
    
    // Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
    
    /* 
    @note
        RUST: Challenge!
    
        fn main() {
            let x = (let y = 6);
        }
        
        or 
        
        fn main() {
            let x = y = 6;
        }
        
     */
    
    
    // expression eg.
    // Note that the x + 1 line doesn’t have a semicolon at the end
    // expression do not include ending semicolons.
    // @todo : post idea.
    // adding semicolons (;) to the end of expression, you turn it to statememt
    let y = {
        let x = 3;
        x + 2
    };
    
    println!("the value y is {}", y);
    
    
    // Functions with Return Values
    // Functions can return values to the code that calls them. 
    // We don’t name return values, but we must declare their type after an arrow (->). 
    // In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function
    // You can return early from a function by using the return keyword and specifying a value,
    
    let number_five = get_number_five();
    
    println!("Number of value {}", number_five);
    
}

fn get_number_five()-> i32 {
    5
}

fn print_value_units(value: i32, label: char) {
    println!("Enter value is => {value}{label}");
}

fn another_function(x: u32) {
    println!("Another function - args {}", x);
}
