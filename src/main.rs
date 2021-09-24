//file ~/IdeaProjects/BadRust/src/main.rs

use std::io;

fn main() {

    let z = "a";
    let x = 5;
    let y = 6;

    println!("welcome to the black mesa facility at sector {} {} {}", x, y, z);

    println!("say number"); //prints message

    let mut number = String::new(); //makes a new variable following a new string as well as being mutual

    io::stdin() //input output standard input
        .read_line(&mut number) //this reads the line user inputs using earlier mutual string variable "guess"
        .expect("u failed to read"); //for funsies

    println!("number: {}", number); //prints a message and then uses the mutual string variable "guess"
}