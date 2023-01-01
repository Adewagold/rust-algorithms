use std::io;
use rand::Rng;
use std::cmp::Ordering;


pub fn guess(){

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number! in 10 attempts");
    let mut attempt_count = 0;
    println!("Please input your guess");
    loop{
        if attempt_count == 10 {
            print!("You lose");
            break;
        }
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        println!("You guessed: {guess}");
    
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
        println!("You have {} attempts left", 10 - attempt_count);
        attempt_count+=1;
    }
    

}