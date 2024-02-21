use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number 🤔!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number 😵‍💫 \n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess a larger number 🥱\n"),
            Ordering::Greater => println!("Guess a smaller number 😪\n"),
            Ordering::Equal => {
                println!("\nYou Win! 🎖️ 🎊\n");
                break;
            }
        }
    }
}
