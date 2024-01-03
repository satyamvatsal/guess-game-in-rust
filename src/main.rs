use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main()
{
    let mut step_count:u32 = 0;
    println!("Guessing Game");
    println!("Enter your guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter correct number");
                continue;
            }
        };
        step_count+=1;
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
            {
                println!("You won");
                break;
            }
        }
        
    }
    println!("You took {step_count} steps");


}
