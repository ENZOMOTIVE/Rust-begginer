use std::io;

fn main(){
    let secret_number= 5;
    let mut guess = String::new();
    
    println!("\nYo!! Bro wanna play........ ");
    println!("\nGuess the Number from 1 to 10........ \n");

    io::stdin().read_line(&mut guess).expect("Failed to read input");
    
    let guess: i32 = guess.trim().parse().expect("Enter the guess");
    
    if guess == secret_number{
        println!(".............YOU WON BRO...............");
    }
    else{
        println!("............. HIHI YOU LOOSE...........");
    }
}
