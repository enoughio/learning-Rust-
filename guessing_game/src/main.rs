use std::io;


fn main() {

    println!("guess the number =---------= : ");
    println!("");
    println!("Enter your guess : ");

    let mut guess = String::new();   // creates a new empty instace of a string 
    
    // getting an input from user 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    
    println!("Your guess {}" ,guess);

    

}