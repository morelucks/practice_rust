use std::io::{self, Write};
use std::mem;

pub fn run (){
    // Prompt the user to enter something
    print!("Please enter some text: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
   
    // let mem2 = mem::size_of_val(&input);

    // Remove the trailing newline character
    input = input.trim().to_string();

    // Check the size of the input
    let size = input.len();
    let mem1 = mem::size_of_val(&input);
    // Display the size to the user
    println!("The size of your input is: {} characters", size);

    println!("The size of your input on memory1 is: {} byte", mem1);

    // println!("The size of your input on memory2 is: {} byte", mem2);

}
