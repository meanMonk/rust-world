// REPETITION WITH LOOPS.
fn main() {
    // Rust has 3 kinds of Loops.
    // `loop`, `while` ,`for`
    // Ways to Break & Skip loops.
    // `break` -> keyword within the loop tell the program when to stop executing the loop.
    // `continue` -> keyword tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

    println!("#### Loops In Rust - loop ####");

    let mut counter = 0;

    // note: The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    'counting_loop: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_loop;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    
    // CONDITAIONAL loop with `while`
    
    let mut magic_number =  3;
    
    while magic_number != 0 {
        println!("magic number {magic_number}");
        
        magic_number -=1;
    }
    
    println!("LIFTOFF!!!");
    println!("Looping through the collection!!");
    
    // Looping through the collection.
    // approach is error prone so better to go `for` loop.
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    
    // `for` loop execute some code for each time in collection.
    
    let mut shoe_sizes = [10,20,30,40];
    
    shoe_sizes.reverse();
    
    for element in shoe_sizes {
        println!("the sizes of shoe's {element}");
    }
    
    // loops with range.
    for count_down_item in (1..=3).rev() {
        println!("count_down_item {count_down_item}");
    }
    println!("LIFTOFF!!")
    
}
