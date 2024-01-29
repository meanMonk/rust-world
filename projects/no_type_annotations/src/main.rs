// Data Types.
// Rust is statically typed language.
// which means that it must know the types of all vars at compile time.
/*
   DataTypes:
    - tells rust what kind of data is being specified.
    - two types:
          1. Scalar
          2. Compound
    - we neeed to add type annotation while converting from string to number using parse()
    - else rust compiler with display error.
*/

/*
 Scalar Types:
   - Represents single value
   - Primary scalar types:
       1. Integers -> should be unsigned integer (singed starts with `i` instead of `u`)


     // Each signed variant can store numbers from -(2e(n - 1)) to 2e(n - 1) - 1

     - `isize` & `usize`

     eg:
       - Number literals can also use `_` as a visual separator to make
       - the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.
        1_000 == 1000


       2. Floating-point numbers



       3. Booleans
       4. Characters

*/

fn main() {
    // Floating Point Numbers:
    // Rusts floating-point types are f32 & f64
    // default is f64
    // all the floating points are signed.

    let x = 2.3; //

    let y: f32 = 3.000003; //

    println!("Hello, world! {x} -y goes here {y}");

    /*
       #Numeric Operations:
        - basic math operations.
         - add / sub / div / remainder / multiplication
        - integer div truncates toward zero to the nearest integer.
    */

    // addition.
    let sum = 4 + 6;
    // substraction
    let difference = 95.5 - 4.3;
    // multiplication.
    let product = 4 * 6;
    // division.
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //

    println!(" => {sum} => {difference} => {product} => {quotient} => {truncated} ");
    
    /* # Boolean Types */
    
    let t = true;
    
    let f: bool = false; // with explicity type annotation.
    
    println!("boolean {f} => {t}");
    
    /* #Character Type 
         - Rust's char type is the langs most primitive alphabetic type.
         eg.
         
         - Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes
         - Rust `char` type is four bytes in size 
    
    eg:
    
    let c = 'z';
    
    let z : char = 'Z';
    
    let heart_eyed_cat = '😻';
     */
    
    /*  #Compound Types
        - can group multiple values into one type.
        - rust has two primitive compound types.
          1. Tuples - single compound element
          2. Array
    */
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // to get the individual value we use pattern matching to destructrure a tuple value.
    // destructuring.
    let (x,y,z) = tup;
    println!("tuple values {x} {y} {z}");
    // Can access a tuple element directly by using a period (.) followed by the index of the value we want to access. 
    println!("tuple value by index and . operator {} {} {}",tup.0, tup.1, tup.2);
    
    
    // Array
    // - array is single chunck of memory of known, fixed size that can be allocated on stack.
    // - can have collection of multiple values with an array.
    // - every element must have of same type.
    // - array in Rust should have fixed length.
    // - are usefull when you want your data on stack rather than heap.
    // - for flexible length we can use vector type can shrink and grow in size.
    // eg.
    let array = [1,2,3,4,5];
    // you can write array's type like square brackets with type;len
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    // You can also initialize an array to contain the same value for each element by specifying the initial value,
    let array_of_4 = [4;5];
    
    // Accessing array elements.
    // array[0]; months[4];
    
    println!("array fist {}  months 2 {} array_of_4 {}", array[0], months[3], array_of_4[2]);
    
}
