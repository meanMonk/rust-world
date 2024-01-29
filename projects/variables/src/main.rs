// Variables & mutability.

/* 
    - Bydefualt variables are immutable.
    - takes advantage of the safety and easy concurrency that Rust offers
    - you still have option to make your variables to mutable using `mut` keyword

*/

// Constants

/* 
    - like immutable variables, constants are values that are bount to a name & are not allowed to change.
    - differen between constants & variables.
    
    1. you are not allowed to use `mut` with constants.
    2. they are always immutable
    3. can be declare using `const` keyword instead of `let`
    4. type of the value must be annotated !important.
    5. constants may be set only to constant expression, not the result of a value
    that could only be computed at runtie.
    6. Constants are valid for the entire time a program runs, within the scope in which they were declared
*/

// Shadowing.

/*  
    #shadowing.
      - The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name
    
*/

fn main() {
    let mut  x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    
    // constant example.
    const THREE_HOURS_IN_SECONDS : u32  =  60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS {}",THREE_HOURS_IN_SECONDS);
    
    // Shadowing.
    // Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword
    
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }
    
    println!("The value of x is : {x}");
    
    let spaces = "    ";
    
    let spaces =  spaces.len();
    
    println!("Space values is {spaces}");
    
}
