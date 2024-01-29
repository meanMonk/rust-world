/*
    - basic challenge provided harcoded months name
    - ask user to enter the number and show corresponding month full name.

*/

use std::io;

fn main() {
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    loop {
        println!("Please enter the month index!");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("failed to read line!");

        let index: usize = match index.trim().parse(){
            Ok(num) => num,
            Err(_) => break,
        };

        println!("{} is your month!", months[index]);
    }
}
